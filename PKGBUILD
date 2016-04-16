# Maintainer: Adam <sector-f on github>
pkgname=prepend-git
_pkgname=prepend
pkgver=1.0
pkgrel=1
pkgdesc="CLI utility that prepends data to a file"
arch=('any')
url="https://github.com/sector-f/prepend"
license=('unknown')
provides=($_pkgname)
conflicts=($_pkgname)
makedepends=('git' 'rust' 'cargo')
source=("$pkgname::git+https://github.com/sector-f/prepend.git")
md5sums=('SKIP')

pkgver() {
  cd "$pkgname"
  git describe --tags | sed 's/\([^-]*-g\)/r\1/;s/-/./g'
}

build() {
	cd "$pkgname"
	cargo build --release
}

package() {
	cd "$pkgname"
	make DESTDIR="${pkgdir}" install
}
