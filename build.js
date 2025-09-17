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
  let changelogContent = fs.readFileSync(changelogPath, 'utf8');
  const newEntry = `rup (${version}-1) unstable; urgency=medium

  * Update.

 -- Esther <esther24072006@gmail.com>  ${new Date().toISOString().split('T')[0]} 00:00:00 +0000

`;
  changelogContent = newEntry + changelogContent;
  fs.writeFileSync(changelogPath, changelogContent);

  console.log('Installing dependencies...');
  try {
    execSync('pacman -S --noconfirm mingw-w64-gcc', { stdio: 'inherit' });
  } catch (e) {
    console.log('Failed to install mingw-w64-gcc. Windows build may fail.');
  }

  console.log('Installing targets...');
  execSync('rustup target add x86_64-pc-windows-gnu', { stdio: 'inherit' });

  console.log('Building for Debian...');
  execSync('cargo deb', { stdio: 'inherit' });

  console.log('Building for Arch Linux...');
  execSync('makepkg -f', { stdio: 'inherit' });

  console.log('Building for Windows (x86_64-pc-windows-gnu)...');
  execSync('cargo build --release --target x86_64-pc-windows-gnu', { stdio: 'inherit' });

  console.log('Moving build artifacts to builds/ directory...');
  execSync('rm -rf builds/', { stdio: 'inherit' });
  execSync('mkdir -p builds', { stdio: 'inherit' });
  try {
    execSync('mv target/debian/rup_' + version + '-1_amd64.deb builds/', { stdio: 'inherit' });
  } catch (e) {
    console.log('Debian package not found');
  }
  try {
    execSync('mv rup-' + version + '-1-x86_64.pkg.tar.zst builds/', { stdio: 'inherit' });
  } catch (e) {
    console.log('Arch package not found');
  }
  try {
    execSync('mv target/x86_64-pc-windows-gnu/release/rup.exe builds/', { stdio: 'inherit' });
  } catch (e) {
    console.log('Windows binary not found');
  }

  // Generate checksums
  const crypto = require('crypto');
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

  console.log('Build complete.');
  console.log(`Debian package: builds/rup_${version}-1_amd64.deb`);
  console.log(`Arch package: builds/rup-${version}-1-x86_64.pkg.tar.zst`);
  console.log('Windows binary: builds/rup.exe');
  console.log('Checksums saved to builds/checksums.txt');
});
