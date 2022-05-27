pkgname=go-thumbnailer
pkgver=0.1.0
pkgrel=1
pkgdesc="A cover thumbnailer written in Go for performance and reliability."
arch=('any')
url="https://github.com/donovanglover/go-thumbnailer"
license=('MIT')
depends=('libvips')
makedepends=('go')
source=('main.go'
        'go.thumbnailer')
sha256sums=('3f11d27cd9398a4f1b7ef82cc2fc77adce68acff3bcfd1f6ee0c48e8dd37f605'
            'e78679c7d5981338afa3595e1ae6104176bf7b32819b73bb894630f5e97e42c5')

build() {
  export CGO_CPPFLAGS="${CPPFLAGS}"
  export CGO_CFLAGS="${CFLAGS}"
  export CGO_CXXFLAGS="${CXXFLAGS}"
  export CGO_LDFLAGS="${LDFLAGS}"
  export GOFLAGS="-buildmode=pie -trimpath -ldflags=-linkmode=external -mod=readonly -modcacherw"

  go build -o go-thumbnailer
}

package() {
  install -Dm755 go-thumbnailer "$pkgdir"/usr/bin/go-thumbnailer
  install -Dm644 go.thumbnailer "$pkgdir"/usr/share/thumbnailers/go.thumbnailer
}
