# Instalación - Tisma Polkit Agent

Este documento describe cómo instalar automáticamente Tisma Polkit Agent con todas sus dependencias.

## Requisitos Previos

- Linux (Arch Linux, Debian/Ubuntu, o Fedora/RHEL)
- Conexión a internet
- Acceso a `sudo` para instalar paquetes

## Instalación Rápida (Recomendado)

```bash
# 1. Clonar el repositorio
git clone https://github.com/tu-usuario/tisma-agent-polkit.git
cd tisma-agent-polkit

# 2. Ejecutar el script de instalación
chmod +x install.sh
./install.sh
```

El script detectará automáticamente tu distribución Linux e instalará:
- ✅ Rust y Cargo
- ✅ GTK4 y GTK4 development files
- ✅ Libadwaita (UI moderna)
- ✅ Vala compiler
- ✅ D-Bus development files
- ✅ pkg-config y GLib
- ✅ Compilará el proyecto
- ✅ Instalará el binario
- ✅ Habilitará el servicio systemd

## Distribuciones Soportadas

### Arch Linux / Manjaro
```bash
./install.sh
```
Instalará paquetes vía `pacman`

### Debian / Ubuntu
```bash
./install.sh
```
Instalará paquetes vía `apt-get`

### Fedora / RHEL / CentOS
```bash
./install.sh
```
Instalará paquetes vía `dnf`

## Instalación Manual

Si el script automático no funciona, puedes instalar manualmente:

### Arch Linux
```bash
sudo pacman -S base-devel rustup cargo gtk4 libadwaita vala dbus pkg-config glib2

# Luego compilar e instalar
cargo build --release
sudo make install
```

### Debian / Ubuntu
```bash
sudo apt-get update
sudo apt-get install -y build-essential rustup cargo libgtk-4-dev \
    libadwaita-1-dev valac dbus libdbus-1-dev pkg-config libglib2.0-dev

# Luego compilar e instalar
cargo build --release
sudo make install
```

### Fedora / RHEL
```bash
sudo dnf install -y @development-tools rustup cargo gtk4-devel \
    libadwaita-devel vala dbus-devel pkg-config glib2-devel

# Luego compilar e instalar
cargo build --release
sudo make install
```

## Uso Después de Instalación

### Verificar estado
```bash
systemctl --user status tisma-agent-polkit
```

### Ver logs en tiempo real
```bash
journalctl --user -u tisma-agent-polkit -f
```

### Detener el servicio
```bash
systemctl --user stop tisma-agent-polkit
```

### Reiniciar el servicio
```bash
systemctl --user restart tisma-agent-polkit
```

## Desinstalación

```bash
# Desactivar servicio
systemctl --user disable tisma-agent-polkit
systemctl --user stop tisma-agent-polkit

# Desinstalar archivos
sudo make uninstall
```

## Solución de Problemas

### "Distribución no soportada"
El script detectó que usas una distribución no soportada. Instala manualmente las dependencias listadas arriba.

### "cargo no encontrado"
```bash
# Instala Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

### "valac no encontrado"
```bash
# Arch Linux
sudo pacman -S vala

# Debian/Ubuntu
sudo apt-get install valac

# Fedora
sudo dnf install vala
```

### Error compilando "gtk not found"
```bash
# Arch Linux
sudo pacman -S gtk4

# Debian/Ubuntu
sudo apt-get install libgtk-4-dev

# Fedora
sudo dnf install gtk4-devel
```

### Error al registrar en D-Bus
El servicio requiere que D-Bus esté corriendo. Generalmente está activo por defecto. Si no:

```bash
# Iniciar D-Bus
sudo systemctl start dbus

# O para usuario (recomendado)
systemctl --user start dbus
```

## Dependencias Instaladas

| Paquete | Propósito |
|---------|-----------|
| `rust` / `cargo` | Compilador y gestor de paquetes |
| `gtk4` | Interfaz gráfica moderna |
| `libadwaita` | Tema y componentes UI |
| `vala` | Lenguaje para UI (compilado a C) |
| `dbus` | Sistema de comunicación entre procesos |
| `pkg-config` | Resolver librerías del sistema |
| `glib2` | Librería de base para GTK |

## Arquitectura de Instalación

```
install.sh (detección de distro)
├── Arch Linux → pacman
├── Debian/Ubuntu → apt-get
└── Fedora/RHEL → dnf
        ↓
Instala dependencias de compilación
        ↓
cargo build --release
        ↓
sudo make install
        ↓
systemctl --user enable/start
```

## Variables de Entorno

Tras instalación, se pueden configurar:

```bash
# Log level (debug, info, warn, error)
export RUST_LOG=tisma_agent_polkit=debug
systemctl --user restart tisma-agent-polkit
```

## Soporte

Si tienes problemas:

1. Verifica que tu distribución sea soportada
2. Asegúrate de tener acceso a `sudo`
3. Revisa los logs: `journalctl --user -u tisma-agent-polkit -f`
4. Abre un issue en el repositorio con los logs

---

**Última actualización:** 2026-04-08  
**Mantenido por:** GitHub Copilot
