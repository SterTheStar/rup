const fs = require('fs');
const path = require('path');
const { execSync } = require('child_process');
const readline = require('readline');

const rl = readline.createInterface({
  input: process.stdin,
  output: process.stdout
});

rl.question('Enter the version: ', (version) => {
  rl.close();

  const results = [];

  // Update Cargo.toml
  const cargoPath = 'Cargo.toml';
  let cargoContent = fs.readFileSync(cargoPath, 'utf8');
  cargoContent = cargoContent.replace(/version = "[^"]*"/, `version = "${version}"`);
  fs.writeFileSync(cargoPath, cargoContent);

  // Update PKGBUILD
  const pkgbuildPath = 'PKGBUILD';
  let pkgbuildContent = fs.readFileSync(pkgbuildPath, 'utf8');
  pkgbuildContent = pkgbuildContent.replace(/pkgver=[^\n]*/, `pkgver=${version}`);
  fs.writeFileSync(pkgbuildPath, pkgbuildContent);

  // Update debian/changelog
  const changelogPath = 'debian/changelog';
  if (fs.existsSync(changelogPath)) {
    let changelogContent = fs.readFileSync(changelogPath, 'utf8');
    const newEntry = `rup (${version}-1) unstable; urgency=medium

  * Update.

 -- Esther <esther24072006@gmail.com>  ${new Date().toISOString().split('T')[0]} 00:00:00 +0000

`;
    changelogContent = newEntry + changelogContent;
    fs.writeFileSync(changelogPath, changelogContent);
  } else {
    console.log('Skipping debian/changelog update (file not found).');
  }

  console.log('Installing dependencies...');
  try {
    execSync('pacman -S --noconfirm mingw-w64-gcc', { stdio: 'inherit' });
  } catch (e) {
    console.log('Failed to install mingw-w64-gcc. Windows build may fail.');
  }

  console.log('Installing targets...');
  try {
    execSync('rustup target add x86_64-pc-windows-gnu', { stdio: 'inherit' });
  } catch (e) {
    console.log('Skipping Windows target: `rustup` not found or failed. Install Rust via rustup and run `rustup target add x86_64-pc-windows-gnu` if you need Windows builds.');
  }

  console.log('Ensuring cargo-deb is installed...');
  let cargoDebAvailable = true;
  try {
    execSync('cargo deb --version', { stdio: 'ignore' });
  } catch (e) {
    cargoDebAvailable = false;
  }

  if (!cargoDebAvailable) {
    try {
      console.log('`cargo-deb` not found. Installing with `cargo install cargo-deb` (this may take a while)...');
      execSync('cargo install cargo-deb', { stdio: 'inherit' });
      cargoDebAvailable = true;
    } catch (e) {
      console.log('Failed to install `cargo-deb`. Debian package will not be built.');
    }
  }

  console.log('Building for Debian...');
  try {
    if (cargoDebAvailable) {
      execSync('cargo deb', { stdio: 'inherit' });
      results.push({ name: 'debian', success: true });
    } else {
      console.log('Skipping Debian build because `cargo-deb` is not available.');
      results.push({ name: 'debian', success: false, error: '`cargo-deb` not available' });
    }
  } catch (e) {
    console.log('Failed to build Debian package. Make sure `cargo-deb` is installed (e.g. `cargo install cargo-deb`).');
    results.push({ name: 'debian', success: false, error: e.message || 'cargo deb failed' });
  }

  console.log('Building for Arch Linux...');
  try {
    execSync('makepkg -f', { stdio: 'inherit' });
  } catch (e) {
    console.log('Failed to build Arch Linux package with makepkg.');
    results.push({ name: 'arch', success: false, error: e.message || 'makepkg failed' });
  }

  console.log('Building for Windows (x86_64-pc-windows-gnu)...');
  try {
    execSync('cargo build --release --target x86_64-pc-windows-gnu', { stdio: 'inherit' });
  } catch (e) {
    console.log('Failed to build Windows target. Ensure the `x86_64-pc-windows-gnu` target and MinGW toolchain are installed.');
    results.push({ name: 'windows-binary', success: false, error: e.message || 'cargo build failed' });
  }

  console.log('Installing NSIS for Windows installer...');
  try {
    execSync('pacman -S --noconfirm nsis', { stdio: 'inherit' });
  } catch (e) {
    console.log('Failed to install NSIS. Windows installer will not be created.');
  }

  console.log('Generating NSIS installer script...');
  const nsiContent = `!include "MUI2.nsh"

Name "rup ${version}"
OutFile "rup-installer-${version}.exe"
InstallDir "$PROGRAMFILES\\rup"
InstallDirRegKey HKLM "Software\\rup" ""
!insertmacro MUI_PAGE_LICENSE "LICENSE"
!insertmacro MUI_PAGE_DIRECTORY
!insertmacro MUI_PAGE_INSTFILES
!insertmacro MUI_UNPAGE_CONFIRM
!insertmacro MUI_UNPAGE_INSTFILES
!insertmacro MUI_LANGUAGE "English"

Section "Main"
  SetOutPath $INSTDIR
  File "builds\\rup.exe"
  WriteRegStr HKLM "Software\\rup" "" $INSTDIR
  WriteUninstaller "$INSTDIR\\uninstall.exe"
  ; Add to PATH
  ReadRegStr $R0 HKLM "SYSTEM\\CurrentControlSet\\Control\\Session Manager\\Environment" "PATH"
  StrCmp $R0 "" 0 +2
  WriteRegExpandStr HKLM "SYSTEM\\CurrentControlSet\\Control\\Session Manager\\Environment" "PATH" "$INSTDIR"
  GoTo +3
  WriteRegExpandStr HKLM "SYSTEM\\CurrentControlSet\\Control\\Session Manager\\Environment" "PATH" "$R0;$INSTDIR"
  SendMessage \${HWND_BROADCAST} \${WM_WININICHANGE} 0 "STR:Environment" /TIMEOUT=5000
SectionEnd

Section "Uninstall"
  Delete "$INSTDIR\\rup.exe"
  Delete "$INSTDIR\\uninstall.exe"
  RMDir "$INSTDIR"
  DeleteRegKey HKLM "Software\\rup"
  ; Remove from PATH
  ReadRegStr $R0 HKLM "SYSTEM\\CurrentControlSet\\Control\\Session Manager\\Environment" "PATH"
  StrCmp $R0 "" 0 +2
  GoTo +4
  Push "$INSTDIR"
  Call un.RemoveFromPath
  Pop $R0
  StrCmp $R0 "0" 0 +2
  WriteRegExpandStr HKLM "SYSTEM\\CurrentControlSet\\Control\\Session Manager\\Environment" "PATH" "$R0"
  SendMessage \${HWND_BROADCAST} \${WM_WININICHANGE} 0 "STR:Environment" /TIMEOUT=5000
SectionEnd

Function un.RemoveFromPath
  Exch $0
  Push $1
  Push $2
  Push $3
  Push $4
  Push $5
  Push $6

  ReadRegStr $1 HKLM "SYSTEM\\CurrentControlSet\\Control\\Session Manager\\Environment" "PATH"
  StrCpy $5 $1 1 -1
  StrCmp $5 ";" 0 +2
  StrCpy $1 $1 -1
  StrCpy $0 "$0;"
  Push $0
  Push $1
  Call un.StrStr
  Pop $2
  StrCmp $2 "" 0 +2
  StrCpy $3 $1
  GoTo +5
  StrCpy $3 $1 $2
  StrCpy $4 $2 "" 1
  StrCpy $4 $4 "" 1
  StrCpy $3 "$3$4"
  StrCpy $5 $3 1 -1
  StrCmp $5 ";" 0 +2
  StrCpy $3 $3 -1
  StrCpy $5 $3 1
  StrCmp $5 ";" 0 +2
  StrCpy $3 $3 "" 1

  WriteRegExpandStr HKLM "SYSTEM\\CurrentControlSet\\Control\\Session Manager\\Environment" "PATH" "$3"
  SendMessage \${HWND_BROADCAST} \${WM_WININICHANGE} 0 "STR:Environment" /TIMEOUT=5000

  Pop $6
  Pop $5
  Pop $4
  Pop $3
  Pop $2
  Pop $1
  Pop $0
FunctionEnd

Function un.StrStr
  Exch $R1
  Exch
  Exch $R2
  Push $R3
  Push $R4
  Push $R5
  StrLen $R3 $R1
  StrCpy $R4 0
  StrLen $R5 $R2
  IntCmp $R5 0 0 0 +3
  StrCpy $R1 ""
  Goto +7
  loop:
    StrCpy $R5 $R2 $R3 $R4
    StrCmp $R5 $R1 done
    IntCmp $R4 $R5 +3
    IntOp $R4 $R4 + 1
    Goto loop
  StrCpy $R1 ""
  done:
  Pop $R5
  Pop $R4
  Pop $R3
  Exch $R2
  Exch
  Exch $R1
FunctionEnd`;
  fs.writeFileSync('installer.nsi', nsiContent);

  console.log('Compiling Windows installer...');
  try {
    execSync('makensis installer.nsi', { stdio: 'inherit' });
  } catch (e) {
    console.log('Failed to compile installer.');
    results.push({ name: 'windows-installer', success: false, error: e.message || 'makensis failed' });
  }

  console.log('Moving build artifacts to builds/ directory...');
  try {
    fs.rmSync('builds', { recursive: true, force: true });
  } catch (e) {
    // ignore
  }
  fs.mkdirSync('builds', { recursive: true });
  try {
    execSync('mv target/debian/rup_' + version + '-1_amd64.deb builds/', { stdio: 'inherit' });
  } catch (e) {
    console.log('Debian package not found');
    // if the move failed but build step said ok, mark as fail here
    results.push({ name: 'debian', success: false, error: 'Debian package file not found' });
  }
  try {
    execSync('mv rup-' + version + '-1-x86_64.pkg.tar.zst builds/', { stdio: 'inherit' });
  } catch (e) {
    console.log('Arch package not found');
    results.push({ name: 'arch', success: false, error: 'Arch package file not found' });
  }
  try {
    execSync('mv target/x86_64-pc-windows-gnu/release/rup.exe builds/', { stdio: 'inherit' });
  } catch (e) {
    console.log('Windows binary not found');
    results.push({ name: 'windows-binary', success: false, error: 'Windows binary file not found' });
  }
  try {
    execSync('mv rup-installer-' + version + '.exe builds/', { stdio: 'inherit' });
  } catch (e) {
    console.log('Windows installer not found');
    results.push({ name: 'windows-installer', success: false, error: 'Windows installer file not found' });
  }

  // Generate checksums
  const crypto = require('crypto');
  try {
    const files = fs.readdirSync('builds/');
    let checksums = '';
    files.forEach(file => {
      if (file !== 'checksums.txt') {
        const filePath = path.join('builds/', file);
        const hash = crypto.createHash('sha256');
        const data = fs.readFileSync(filePath);
        hash.update(data);
        checksums += `${hash.digest('hex')}  ${file}\n`;
      }
    });
    fs.writeFileSync('builds/checksums.txt', checksums);
    results.push({ name: 'checksums', success: true });
  } catch (e) {
    console.log('Failed to generate checksums.');
    results.push({ name: 'checksums', success: false, error: e.message || 'checksums failed' });
  }

  // Write minimal build results summary
  const orderedNames = ['debian', 'arch', 'windows-binary', 'windows-installer', 'checksums'];
  const lines = orderedNames.map((name) => {
    const entry = results.find((r) => r.name === name && r.success === true)
      || results.find((r) => r.name === name);
    if (!entry) {
      return `${name}: not-run`;
    }
    if (entry.success) {
      return `${name}: ok`;
    }
    return `${name}: fail - ${entry.error}`;
  });
  fs.writeFileSync('build.results', lines.join('\n') + '\n');

  console.log('Build complete.');
  console.log(`Debian package: builds/rup_${version}-1_amd64.deb`);
  console.log(`Arch package: builds/rup-${version}-1-x86_64.pkg.tar.zst`);
  console.log('Windows binary: builds/rup.exe');
  console.log(`Windows installer: builds/rup-installer-${version}.exe`);
  console.log('Checksums saved to builds/checksums.txt');
});
