#!/bin/bash

echo "Installing dependencies..."
# For Windows cross-compilation
pacman -S --noconfirm mingw-w64-gcc || echo "Failed to install mingw-w64-gcc. Windows build may fail."
# For macOS, osxcross is required but not installed here

echo "Installing targets..."
rustup target add x86_64-pc-windows-gnu
# Note: macOS target requires osxcross or similar for cross-compilation on Linux
# rustup target add x86_64-apple-darwin || echo "macOS target installation failed. Ensure osxcross is set up for macOS builds."

echo "Building for Debian..."
cargo deb

echo "Building for Arch Linux..."
makepkg -f

echo "Building for Windows (x86_64-pc-windows-gnu)..."
cargo build --release --target x86_64-pc-windows-gnu

# echo "Building for macOS (x86_64-apple-darwin)..."
# cargo build --release --target x86_64-apple-darwin

echo "Moving build artifacts to builds/ directory..."
rm -rf builds/
mkdir -p builds
mv target/debian/rup_0.1.0-1_amd64.deb builds/ || echo "Debian package not found"
mv rup-0.1.0-1-x86_64.pkg.tar.zst builds/ || echo "Arch package not found"
mv target/x86_64-pc-windows-gnu/release/rup.exe builds/ || echo "Windows binary not found"
# mv target/x86_64-apple-darwin/release/rup builds/ || echo "macOS binary not found"

echo "Build complete."
echo "Debian package: builds/rup_0.1.0-1_amd64.deb"
echo "Arch package: builds/rup-0.1.0-1-x86_64.pkg.tar.zst"
echo "Windows binary: builds/rup.exe"
# echo "macOS binary: builds/rup"
