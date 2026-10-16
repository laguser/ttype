# Maintainer: laguser <laguser@users.noreply.github.com>
pkgname=ttype-git
pkgver=0.1.0
pkgrel=1
pkgdesc="Minimalist terminal typing speed test in Rust with typewriter conveyor tape and procedural sound"
arch=('x86_64' 'aarch64')
url="https://github.com/laguser/ttype"
license=('MIT')
depends=('alsa-lib')
makedepends=('cargo' 'git')
provides=('ttype')
conflicts=('ttype')
source=("git+https://github.com/laguser/ttype.git")
sha256sums=('SKIP')

pkgver() {
  cd "$srcdir/ttype"
  printf "r%s.%s" "$(git rev-list --count HEAD)" "$(git rev-parse --short HEAD)"
}

build() {
  cd "$srcdir/ttype"
  cargo build --release --locked
}

package() {
  cd "$srcdir/ttype"
  install -Dm755 "target/release/ttype" "$pkgdir/usr/bin/ttype"
  install -Dm644 "README.md" "$pkgdir/usr/share/doc/ttype/README.md"
}
