# Maintainer: Gaurav Atreya <allmanpride@gmail.com>
pkgname=jisho-chibi
pkgver=0.1.1
pkgrel=1
pkgdesc="Unofficial Minimal Jisho Desktop Application"
arch=('x86_64')
license=('GPL3')
depends=('gcc-libs')
makedepends=('rust' 'cargo')

build() {
	cargo build --release
}

package() {
    cd "$srcdir"
    mkdir -p "$pkgdir/usr/bin" "$pkgdir/usr/share/applications/"
    cp "../target/release/${pkgname}" "$pkgdir/usr/bin/${pkgname}"
    cp "../jisho-chibi.desktop" "$pkgdir/usr/share/applications/"
}
