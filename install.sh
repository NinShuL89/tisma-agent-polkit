#!/bin/bash
# Tisma Polkit Agent - Script de instalación y ejecución

set -e

echo "╔════════════════════════════════════════════════════════════╗"
echo "║     Tisma Polkit Agent - Setup para Arch Linux            ║"
echo "╚════════════════════════════════════════════════════════════╝"
echo ""

# Verificar dependencias
echo "▶ Verificando dependencias..."
deps=("cargo" "rustc" "valac" "pkg-config")
for dep in "${deps[@]}"; do
  if ! command -v "$dep" &> /dev/null; then
    echo "  ✗ $dep no encontrado"
    echo ""
    echo "  Instala las dependencias:"
    echo "  $ sudo pacman -S rustup cargo gtk4 vala libadwaita polkit dbus"
    exit 1
  fi
  echo "  ✓ $dep found"
done

echo ""
echo "▶ Compilando Tisma Polkit Agent..."
cargo build --release

echo ""
echo "▶ Instalándolo..."
sudo make install

echo ""
echo "▶ Habilitando servicio systemd..."
systemctl --user enable tisma-agent-polkit
systemctl --user start tisma-agent-polkit

echo ""
echo "✓ Instalación completada!"
echo ""
echo "Verificar estado:"
echo "  $ systemctl --user status tisma-agent-polkit"
echo ""
echo "Ver logs:"
echo "  $ journalctl --user -u tisma-agent-polkit -f"
