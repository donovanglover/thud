# Maintainer: Donovan Glover <https://donovan.is/>
pkgname=thud
pkgver=0.2.0
pkgrel=1
pkgdesc="Generate directory thumbnails for GTK-based file browsers from images inside them"
arch=('any')
url="https://github.com/donovanglover/thud"
license=('MIT')
depends=('gcc-libs')
makedepends=('cargo')
source=("$pkgname-$pkgver.tar.gz::https://github.com/donovanglover/$pkgname/archive/$pkgver.tar.gz")
sha256sums=('SKIP')

build() {
  cd "$pkgname-$pkgver"

  cargo build --release --locked
}

package() {
  cd "$pkgname-$pkgver"

  install -Dm755 "target/release/thud" "$pkgdir/usr/bin/thud"
  install -Dm644 "assets/thud.thumbnailer" "$pkgdir/usr/share/thumbnailers/thud.thumbnailer"

  install -Dm644 "completions/_thud" "$pkgdir/usr/share/zsh/site-functions/_thud"
  install -Dm644 "completions/thud.bash" "$pkgdir/usr/share/bash-completion/completions/thud"
  install -Dm644 "completions/thud.fish" "$pkgdir/usr/share/fish/vendor_completions.d/thud.fish"
  install -Dm644 "man/thud.1" "$pkgdir/usr/share/man/man1/thud.1"

  install -Dm644 "README.md" "$pkgdir/usr/share/doc/$pkgname/README.md"
  install -Dm644 "LICENSE" "$pkgdir/usr/share/licenses/$pkgname/LICENSE"
}
