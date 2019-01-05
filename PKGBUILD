# Maintainer: Alex Lai <alexlai97@pm.me>

pkgname=tictactoe-rust
pkgver=0.1.3
pkgdesc="Tictactoe written in Rust"
pkgrel=1
arch=('x86_64')
url="https://github.com/alexlai97/tictactoe-rust"
license=('MIT')
#depends=('gcc-libs' 'pcre2')
makedepends=('cargo')
source=("https://github.com/alexlai97/$pkgname/archive/$pkgver.tar.gz")
sha512sums=('67603da3ea3e75c774f8ac3b45efde3f16b5d1e9acb43d36b40bf4052260d8e7c7f4840b0b8a095a4e07e598e8a19d2a62e4f63cb0ed574212a422af9eefa4fe')

build() {
  cd "$pkgname-$pkgver"

  cargo build --release
}

package() {
  cd "$pkgname-$pkgver"

  install -Dm755 "target/release/tictactoe-rust" "$pkgdir/usr/bin/tictactoe-rust"

}
