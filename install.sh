#!/bin/bash
# Tisma Polkit Agent - Script de instalación automática
# Soporta: Arch Linux, Debian/Ubuntu, Fedora/RHEL

set -e

echo "╔════════════════════════════════════════════════════════════╗"
echo "║     Tisma Polkit Agent - Instalación Automática           ║"
echo "╚════════════════════════════════════════════════════════════╝"
echo ""

# Detectar distribución Linux
detect_distro() {
    if [ -f /etc/arch-release ]; then
        echo "arch"
    elif [ -f /etc/debian_version ]; then
        echo "debian"
    elif [ -f /etc/redhat-release ]; then
        echo "fedora"
    else
        echo "unknown"
    fi
}

# Instalar dependencias en Arch Linux
install_arch() {
    echo "▶ Detectado: Arch Linux"
    echo "▶ Instalando dependencias..."
    echo ""
    
    sudo pacman -Syu --noconfirm
    
    # Paquetes de compilación y Rust
    echo "  • Instalando Rust y herramientas de compilación..."
    sudo pacman -S --noconfirm base-devel rustup cargo
    
    # GTK4 y dependencias de interfaz
    echo "  • Instalando GTK4 y libadwaita..."
    sudo pacman -S --noconfirm gtk4 libadwaita
    
    # Vala compilador
    echo "  • Instalando Vala..."
    sudo pacman -S --noconfirm vala
    
    # D-Bus
    echo "  • Instalando D-Bus..."
    sudo pacman -S --noconfirm dbus
    
    # pkg-config para resolver librerías
    echo "  • Instalando pkg-config..."
    sudo pacman -S --noconfirm pkg-config
    
    # GIO y GLib
    echo "  • Instalando GLib y GIO..."
    sudo pacman -S --noconfirm glib2
    
    echo "  ✓ Todas las dependencias instaladas"
}

# Instalar dependencias en Debian/Ubuntu
install_debian() {
    echo "▶ Detectado: Debian/Ubuntu"
    echo "▶ Instalando dependencias..."
    echo ""
    
    sudo apt-get update
    
    # Paquetes de compilación y Rust
    echo "  • Instalando Rust y herramientas de compilación..."
    sudo apt-get install -y build-essential rustup cargo
    
    # GTK4 y dependencias
    echo "  • Instalando GTK4 y dependencias..."
    sudo apt-get install -y libgtk-4-dev libgtk-4-0
    
    # Libadwaita (para UI moderna)
    echo "  • Instalando Libadwaita..."
    sudo apt-get install -y libadwaita-1-dev
    
    # Vala compilador
    echo "  • Instalando Vala..."
    sudo apt-get install -y valac
    
    # D-Bus
    echo "  • Instalando D-Bus..."
    sudo apt-get install -y dbus libdbus-1-dev
    
    # pkg-config
    echo "  • Instalando pkg-config..."
    sudo apt-get install -y pkg-config
    
    # GLib y GIO
    echo "  • Instalando GLib y GIO..."
    sudo apt-get install -y libglib2.0-dev libgio-2.0-0
    
    echo "  ✓ Todas las dependencias instaladas"
}

# Instalar dependencias en Fedora/RHEL
install_fedora() {
    echo "▶ Detectado: Fedora/RHEL"
    echo "▶ Instalando dependencias..."
    echo ""
    
    sudo dnf install -y dnf-plugins-core
    
    # Paquetes de compilación y Rust
    echo "  • Instalando Rust y herramientas de compilación..."
    sudo dnf install -y @development-tools rustup cargo
    
    # GTK4
    echo "  • Instalando GTK4..."
    sudo dnf install -y gtk4-devel
    
    # Libadwaita
    echo "  • Instalando Libadwaita..."
    sudo dnf install -y libadwaita-devel
    
    # Vala
    echo "  • Instalando Vala..."
    sudo dnf install -y vala
    
    # D-Bus
    echo "  • Instalando D-Bus..."
    sudo dnf install -y dbus-devel
    
    # pkg-config
    echo "  • Instalando pkg-config..."
    sudo dnf install -y pkg-config
    
    # GLib
    echo "  • Instalando GLib..."
    sudo dnf install -y glib2-devel
    
    echo "  ✓ Todas las dependencias instaladas"
}

# Función principal
main() {
    # Detectar distro
    DISTRO=$(detect_distro)
    
    case "$DISTRO" in
        arch)
            install_arch
            ;;
        debian)
            install_debian
            ;;
        fedora)
            install_fedora
            ;;
        *)
            echo "✗ Distribución no soportada"
            echo ""
            echo "Distribuciones soportadas:"
            echo "  • Arch Linux"
            echo "  • Debian/Ubuntu"
            echo "  • Fedora/RHEL"
            echo ""
            echo "Instala manualmente:"
            echo "  - Rust/Cargo"
            echo "  - GTK4 development files"
            echo "  - Vala compiler"
            echo "  - D-Bus development files"
            echo "  - pkg-config"
            exit 1
            ;;
    esac
    
    echo ""
    echo "▶ Verificando dependencias clave..."
    
    # Verificar dependencias instaladas
    deps=("cargo" "rustc" "valac" "pkg-config")
    for dep in "${deps[@]}"; do
        if ! command -v "$dep" &> /dev/null; then
            echo "  ✗ $dep no encontrado"
            echo "  Intenta ejecutar nuevamente o instala manualmente"
            exit 1
        fi
        echo "  ✓ $dep disponible"
    done
    
    echo ""
    echo "▶ Compilando Tisma Polkit Agent (Release)..."
    cargo build --release
    
    echo ""
    echo "▶ Instalando binario..."
    sudo make install
    
    echo ""
    echo "▶ Habilitando servicio systemd..."
    systemctl --user daemon-reload
    systemctl --user enable tisma-agent-polkit
    systemctl --user start tisma-agent-polkit
    
    echo ""
    echo "╔════════════════════════════════════════════════════════════╗"
    echo "║            ✓ Instalación completada                       ║"
    echo "╚════════════════════════════════════════════════════════════╝"
    echo ""
    echo "Verificar estado:"
    echo "  $ systemctl --user status tisma-agent-polkit"
    echo ""
    echo "Ver logs en tiempo real:"
    echo "  $ journalctl --user -u tisma-agent-polkit -f"
    echo ""
    echo "Desinstalar:"
    echo "  $ sudo make uninstall"
    echo "  $ systemctl --user disable tisma-agent-polkit"
    echo ""
}

# Ejecutar
main "$@"
