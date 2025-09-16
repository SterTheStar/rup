# Maintainer: Esther <esther24072006@gmail.com>
pkgname=rup
pkgver=0.1.0
pkgrel=1
pkgdesc="A CLI tool for uploading files to various APIs"
arch=('x86_64')
url="https://github.com/yourusername/rup"
license=('GPL3')
depends=('openssl' 'gcc-libs')
makedepends=('cargo')
options=('!debug')
builddir="$startdir"

build() {
  cargo build --release --locked
}

package() {
  cd "$startdir"
  install -Dm755 "target/release/rup" "$pkgdir/usr/bin/$pkgname"
}
