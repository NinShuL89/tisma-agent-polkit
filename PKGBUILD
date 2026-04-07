# Maintainer: Tisma <your-email@example.com>
pkgname=tisma-agent-polkit
pkgver=0.1.0
pkgrel=1
pkgdesc="PolicyKit Agent en Rust con UI en Vala para Arch Linux"
arch=('x86_64' 'aarch64')
url="https://github.com/yourusername/tisma-agent-polkit"
license=('MIT' 'Apache-2.0')
depends=('gtk4' 'libadwaita' 'polkit' 'vala' 'dbus')
makedepends=('rustup' 'cargo' 'git')
options=('!lto')

source=("git+https://github.com/yourusername/tisma-agent-polkit.git#branch=main")
sha256sums=('SKIP')

build() {
  cd "$pkgname"
  cargo build --release
}

check() {
  cd "$pkgname"
  cargo test --release
}

package() {
  cd "$pkgname"
  
  # Instalar binario
  install -Dm755 target/release/tisma-agent-polkit \
    "$pkgdir/usr/bin/tisma-agent-polkit"
  
  # Instalar servicio systemd
  install -Dm644 systemd/tisma-agent-polkit.service \
    "$pkgdir/usr/lib/systemd/user/tisma-agent-polkit.service"
  
  # Instalar descripción D-Bus
  install -Dm644 dbus/org.tisma.PolkitAgent.xml \
    "$pkgdir/usr/share/dbus-1/interfaces/org.tisma.PolkitAgent.xml"
  
  # Instalar políticas polkit
  install -Dm644 policies/org.tisma.policy \
    "$pkgdir/usr/share/polkit-1/actions/org.tisma.policy"
  
  # Instalar documentación
  install -Dm644 README.md \
    "$pkgdir/usr/share/doc/$pkgname/README.md"
}
