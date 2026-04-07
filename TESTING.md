# Testing Guide - Tisma Polkit Agent

Esta guía cubre cómo testear el proyecto en diferentes distribuciones Linux, con énfasis en **Linux Mint**.

## Tabla de Contenidos

1. [Linux Mint (Debian/Ubuntu-based)](#linux-mint)
2. [Arch Linux](#arch-linux)
3. [Pruebas Generales](#pruebas-generales)
4. [Troubleshooting](#troubleshooting)

---

## Linux Mint

### ✅ Compatibilidad

Linux Mint está basado en Debian/Ubuntu y es **totalmente compatible** con Tisma Polkit Agent. Todos los componentes necesarios están disponibles:

- ✅ Rust & Cargo
- ✅ Vala
- ✅ GTK4
- ✅ PolicyKit
- ✅ D-Bus
- ✅ systemd

### 📦 Instalación de Dependencias

```bash
# Actualizar lista de paquetes
sudo apt update

# Instalar dependencias de compilación
sudo apt install -y \
  rustup \
  cargo \
  gtk-4-dev \
  valac \
  libadwaita-1-dev \
  polkit-1 \
  dbus \
  libdbus-1-dev \
  build-essential \
  pkg-config \
  libssl-dev
```

### 🏗️ Compilación

```bash
cd ~/Documentos/tisma-agent-polkit

# Compilar en modo debug
cargo build

# Compilar en modo release (optimizado)
cargo build --release

# Verificar que compiló correctamente
ls -lh target/release/tisma-agent-polkit
```

### 🚀 Instalación Manual

Para testing sin usar systemd de usuario:

```bash
# Copiar binario
sudo cp target/release/tisma-agent-polkit /usr/local/bin/
sudo chmod +x /usr/local/bin/tisma-agent-polkit

# Copiar interfaz D-Bus
sudo mkdir -p /usr/share/dbus-1/interfaces/
sudo cp dbus/org.tisma.PolkitAgent.xml /usr/share/dbus-1/interfaces/

# Copiar políticas PolicyKit
sudo mkdir -p /usr/share/polkit-1/actions/
sudo cp policies/org.tisma.policy /usr/share/polkit-1/actions/

# Verificar permisos
sudo chown root:root /usr/share/dbus-1/interfaces/org.tisma.PolkitAgent.xml
sudo chown root:root /usr/share/polkit-1/actions/org.tisma.policy
sudo chmod 644 /usr/share/dbus-1/interfaces/org.tisma.PolkitAgent.xml
sudo chmod 644 /usr/share/polkit-1/actions/org.tisma.policy
```

### 🧪 Pruebas Unitarias

```bash
# Ejecutar todos los tests
cargo test

# Tests con output
cargo test -- --nocapture

# Tests de internacionalización
cargo test i18n -- --nocapture

# Un test específico
cargo test test_language_detection
```

### 🌍 Testing de Idiomas

```bash
# Español
LANG=es_ES.UTF-8 ./target/release/tisma-agent-polkit

# English
LANG=en_US.UTF-8 ./target/release/tisma-agent-polkit

# Français
LANG=fr_FR.UTF-8 ./target/release/tisma-agent-polkit

# Deutsch
LANG=de_DE.UTF-8 ./target/release/tisma-agent-polkit

# Português
LANG=pt_PT.UTF-8 ./target/release/tisma-agent-polkit

# Català
LANG=ca_ES.UTF-8 ./target/release/tisma-agent-polkit
```

### 🔧 Testing D-Bus

```bash
# 1. Iniciar el agente en background
/usr/local/bin/tisma-agent-polkit &
AGENT_PID=$!

# 2. Verificar que se registró en D-Bus
dbus-send --print-reply --system / \
  org.freedesktop.DBus.ListNames | \
  grep -i tisma

# 3. Introspect la interfaz
dbus-send --print-reply --system \
  /org/tissma/PolkitAgent \
  org.freedesktop.DBus.Introspect

# 4. Detener el agente
kill $AGENT_PID
```

### 🔓 Testing PolicyKit

```bash
# 1. Testear PolicyKit con un comando simple
pkexec echo "PolicyKit test - autorización exitosa"

# 2. Debería aparecer el diálogo de Tisma (si está corriendo)

# 3. Ver logs del agente
dmesg | grep -i polkit
```

### 📊 Testing de Rendimiento

```bash
# Medir tiempo de startup
time ./target/release/tisma-agent-polkit

# Analizar uso de memoria
valgrind --leak-check=full \
  ./target/release/tisma-agent-polkit

# Benchmarking (si está presente hyperfine)
hyperfine ./target/release/tisma-agent-polkit
```

### 📝 Verificación de Compilación

```bash
# Chequear warnings
cargo build --release 2>&1 | grep "warning:"

# Usar clippy para linting
cargo clippy -- -D warnings

# Formatear código
cargo fmt --check
```

---

## Arch Linux

### 📦 Instalación de Dependencias

```bash
sudo pacman -S rustup cargo gtk4 vala libadwaita polkit dbus base-devel
```

### 🏗️ Compilación

```bash
cargo build --release
```

### 🚀 Instalación

```bash
# Usar Makefile
make release
sudo make install

# O manualmente
sudo cp target/release/tisma-agent-polkit /usr/local/bin/
```

### 🔧 Con systemd Usuario

```bash
# Habilitar servicio
systemctl --user enable tisma-agent-polkit
systemctl --user start tisma-agent-polkit

# Ver logs
journalctl --user -u tisma-agent-polkit -f

# Ver estado
systemctl --user status tisma-agent-polkit
```

---

## Pruebas Generales

### ✅ Checklist de Testing

```
Compilación:
[ ] Compila sin errores
[ ] Sin warnings críticos
[ ] Clippy valida el código
[ ] Tests unitarios pasan

D-Bus:
[ ] Agent se registra en D-Bus
[ ] Interfaz es accessible
[ ] Métodos son callable
[ ] Signals funcionan

PolicyKit:
[ ] Detecta solicitudes de auth
[ ] Abre diálogo correctamente
[ ] Valida credenciales
[ ] Registra intentos

UI (Vala/GTK4):
[ ] Diálogo se abre
[ ] Botones funcionan
[ ] Input de password enmascarado
[ ] Responsive a eventos

Internacionalización:
[ ] Detecta idioma del sistema
[ ] Textos traducidos correctamente
[ ] Fallback a inglés funciona
[ ] Variables en mensajes reemplazan

Performance:
[ ] Startup < 100ms
[ ] No memory leaks
[ ] CPU usage bajo en reposo
[ ] UI responsive
```

### 🔍 Verificación de Archivos

```bash
# Verificar que todos los archivos están presentes
ls -la i18n/*.json                    # 6 idiomas
ls -la vala/*.vala                    # UI files
ls -la src/*.rs                       # Source files
ls -la dbus/*.xml                     # D-Bus definitions
ls -la policies/*.policy              # PolicyKit policies

# Verificar JSON válido
for f in i18n/*.json; do
  echo "Validando $f..."
  python3 -m json.tool "$f" > /dev/null && echo "  ✓ OK" || echo "  ✗ ERROR"
done
```

### 📋 Test Suite Recomendado

```bash
#!/bin/bash
# test-all.sh

set -e

echo "=== Tisma Polkit Agent - Full Test Suite ==="
echo ""

echo "1️⃣  Compilación..."
cargo build --release

echo "2️⃣  Validación de código..."
cargo fmt --check
cargo clippy -- -D warnings

echo "3️⃣  Tests unitarios..."
cargo test

echo "4️⃣  Tests de i18n..."
LANG=es_ES.UTF-8 cargo test i18n
LANG=en_US.UTF-8 cargo test i18n

echo "5️⃣  Verificación de JSON..."
for f in i18n/*.json; do
  python3 -m json.tool "$f" > /dev/null || echo "ERROR: $f"
done

echo "6️⃣  Verificación de archivos..."
test -f target/release/tisma-agent-polkit || (echo "Binary not found!"; exit 1)

echo ""
echo "✅ Todos los tests completados exitosamente!"
```

### 🚀 Ejecutar Test Suite

```bash
chmod +x test-all.sh
./test-all.sh
```

---

## Troubleshooting

### Problema: "valac not found"

**Solución Mint:**
```bash
sudo apt install valac
```

**Solución Arch:**
```bash
sudo pacman -S vala
```

### Problema: "gtk-4 not found"

**Solución Mint:**
```bash
sudo apt install gtk-4-dev libgtk-4-dev
pkg-config --modversion gtk4
```

**Solución Arch:**
```bash
sudo pacman -S gtk4
```

### Problema: "D-Bus not running"

```bash
# Iniciar D-Bus daemon
sudo systemctl start dbus

# En systemd usuario
systemctl --user start dbus.service
```

### Problema: "Permission denied" en /usr/local/bin

```bash
sudo chown root:root /usr/local/bin/tisma-agent-polkit
sudo chmod 755 /usr/local/bin/tisma-agent-polkit
```

### Problema: PolicyKit policy no carga

```bash
# Recargar policies
sudo systemctl restart polkit

# Verificar que el archivo existe
ls -la /usr/share/polkit-1/actions/org.tisma.policy
```

### Problema: "GTK4 version too old"

Verificar versión:
```bash
pkg-config --modversion gtk4

# Mint requiere >= 4.0
# Si es menor, actualizar:
sudo apt upgrade gtk-4-dev
```

### Problema: Vala compilation fails

```bash
# Limpiar y recompilar
cargo clean
cargo build --release 2>&1 | head -50

# Ver generados C files
ls -la target/release/build/*/out/*.c
```

### Debugging Avanzado

```bash
# Rust logs detallados
RUST_LOG=debug cargo run

# Vala debugging
valac --debug vala/polkit_dialog.vala

# D-Bus debugging
DBUS_VERBOSE=1 ./target/release/tisma-agent-polkit

# Strace para syscalls
strace -e trace=dbus -o strace.log ./target/release/tisma-agent-polkit
```

---

## Requisitos Mínimos

| Componente | Versión Mínima | Mint | Arch |
|------------|----------------|------|------|
| Rust | 1.56+ | ✅ | ✅ |
| GTK | 4.0+ | ✅ | ✅ |
| Vala | 0.54+ | ✅ | ✅ |
| D-Bus | 1.12+ | ✅ | ✅ |
| PolicyKit | 0.105+ | ✅ | ✅ |

## Reporting Issues

Si encuentras un problema durante testing:

1. Capturar error completo
2. Incluir distribución y versión
3. Incluir output de:
   ```bash
   rustc --version
   cargo --version
   valac --version
   pkg-config --modversion gtk4
   ```
4. Crear issue con detalles

---

**Última actualización**: Abril 7, 2026

Para más información ver [README.md](README.md) y [TRANSLATIONS.md](TRANSLATIONS.md)
