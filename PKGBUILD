# Maintainer: Adam <sector-f on github>
pkgname=prepend
pkgver=2.1.0
pkgrel=1
pkgdesc="CLI utility that prepends data to a file"
arch=('any')
url="https://github.com/sector-f/prepend"
license=('unknown')
provides=($pkgname)
conflicts=($pkgname)
makedepends=('rust' 'cargo')
source=("https://github.com/sector-f/$pkgname/archive/$pkgver.tar.gz")
md5sums=('SKIP')

build() {
	cd "$pkgname-$pkgver"
	cargo build --release
}

package() {
	cd "$pkgname-$pkgver"
	make DESTDIR="${pkgdir}" install
}
