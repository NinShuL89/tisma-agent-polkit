# Tisma Polkit Agent

Un agente de autenticación PolicyKit robusto y moderno escrito en **Rust** con interfaz gráfica en **Vala + GTK4**, diseñado para distribuciones Arch Linux con tiling window managers.

## Características

- 🦀 **Backend en Rust** - Seguridad de memoria, manejo de errores robusto
- 🎨 **UI en Vala/GTK4** - Interfaz moderna e integrada con el DE
- 🔐 **Compatibilidad con PolicyKit** - Soporte completo para org.freedesktop.PolicyKit1
- ⚡ **Optimizado** - Compilación con LTO, binario pequeño y eficiente
- 🪟 **Tiling WM Friendly** - Compatible con i3, dwm, Sway, etc.
- 📋 **D-Bus Integration** - Comunicación nativa con el sistema
- 🌍 **Multiidioma** - Soporte para 6 idiomas (ES, EN, FR, DE, PT, CA)

## Requisitos

### Arch Linux
```bash
sudo pacman -S rustup cargo gtk4 vala libadwaita polkit dbus
```

### Otros perfiles
```bash
# Debian/Ubuntu
sudo apt install rustc cargo libgtk-4-dev valac libpolkit-gobject-1-dev dbus

# Fedora
sudo dnf install rust cargo gtk4-devel vala polkit-devel dbus-devel
```

## Compilación

### Debug
```bash
cargo build
```

### Release (recomendado)
```bash
cargo build --release
```

### Tareas i18n
```bash
# Listar idiomas soportados
make i18n-list

# Verificar integridad de traducciones JSON
make i18n-check

# Crear nueva traducción (ejemplo: italiano)
make i18n-new LANG_CODE=it LANG_NAME='Italian'
```

## Instalación

### Instalación Automática (Recomendado) ⚡

```bash
# Clona el repositorio
git clone https://github.com/tisma/tisma-agent-polkit.git
cd tisma-agent-polkit

# Ejecuta el script de instalación
chmod +x install.sh
./install.sh
```

El script detecta tu distribución Linux e instala automáticamente:
- ✅ Rust y Cargo
- ✅ GTK4 y development files
- ✅ Vala compiler
- ✅ D-Bus libraries
- ✅ pkg-config y GLib
- ✅ Compila el proyecto
- ✅ Instala el binario
- ✅ Habilita el servicio systemd

**Soporta:** Arch Linux, Debian/Ubuntu, Fedora/RHEL

Ver [INSTALL.md](INSTALL.md) para instrucciones detalladas y instalación manual.

### Instalación Manual (Alternativa)

```bash
# Arch Linux
sudo pacman -S base-devel rustup cargo gtk4 vala libadwaita dbus pkg-config glib2
cargo build --release
sudo make install

# Debian/Ubuntu
sudo apt-get install -y build-essential rustup cargo libgtk-4-dev valac dbus libdbus-1-dev pkg-config libglib2.0-dev
cargo build --release
sudo make install

# Fedora
sudo dnf install -y @development-tools rustup cargo gtk4-devel vala dbus-devel pkg-config glib2-devel
cargo build --release
sudo make install
```

### Habilitar como servicio systemd
```bash
systemctl --user enable tisma-agent-polkit
systemctl --user start tisma-agent-polkit
```

### Verificar estado
```bash
systemctl --user status tisma-agent-polkit
```

## Estructura del Proyecto

```
tisma-agent-polkit/
├── src/
│   ├── main.rs           # Punto de entrada
│   ├── i18n.rs           # Sistema de traducciones
│   ├── polkit.rs         # Lógica de polkit (Rust)
│   └── ui/
│       └── mod.rs        # Integración de UI
├── vala/
│   ├── polkit_dialog.vala       # Interfaz gráfica principal (Vala)
│   └── localized_dialog.vala.example  # Ejemplo de localización
├── i18n/
│   ├── es.json           # Español
│   ├── en.json           # English
│   ├── fr.json           # Français
│   ├── de.json           # Deutsch
│   ├── pt.json           # Português
│   └── ca.json           # Català
├── dbus/
│   └── org.tisma.PolkitAgent.xml  # Descripción D-Bus
├── systemd/
│   └── tisma-agent-polkit.service # Servicio systemd
├── policies/
│   └── org.tisma.policy           # Políticas polkit
├── tools/
│   └── generate_i18n.py          # Script para nuevas traducciones
├── Cargo.toml
├── build.rs              # Script de compilación (compila Vala)
├── Makefile
├── i18n.mk               # Makefile para tareas de i18n
├── README.md
└── TRANSLATIONS.md       # Guía de traducción
```

## Cómo funciona

1. **Startup**: El agente se inicia como servicio D-Bus
2. **Registration**: Se registra como agente de autenticación en PolicyKit
3. **Request**: Cuando se necesita autorización, PolicyKit invoca `BeginAuthentication`
4. **Dialog**: Se abre el diálogo de Vala/GTK4
5. **Auth**: El usuario ingresa su contraseña
6. **Completion**: El resultado se devuelve a PolicyKit

## Desarrollo

### Editar la interfaz gráfica
Editar `vala/polkit_dialog.vala` y recompilar:
```bash
cargo build
```

### Logs
```bash
journalctl --user -u tisma-agent-polkit -f
```

### Debugging
```bash
RUST_LOG=debug cargo run
```

### Traducciones

El proyecto soporta múltiples idiomas automáticamente detectados del sistema:

```bash
# Español
LANG=es_ES.UTF-8 cargo run

# Francés
LANG=fr_FR.UTF-8 cargo run

# Alemán
LANG=de_DE.UTF-8 cargo run
```

Ver [TRANSLATIONS.md](TRANSLATIONS.md) para agregar más idiomas.

**Idiomas disponibles:**
- 🇪🇸 Español (es)
- 🇬🇧 English (en)
- 🇫🇷 Français (fr)
- 🇩🇪 Deutsch (de)
- 🇵🇹 Português (pt)
- 🇪🇸 Català (ca)

## Configuración

La configuración se puede personalizar editando:
- `vala/polkit_dialog.vala` - Apariencia de la UI
- `src/polkit.rs` - Lógica de autenticación
- `policies/org.tisma.policy` - Políticas de autorización

## Testing

Ver [TESTING.md](TESTING.md) para:
- 🧪 Guía completa de testing
- 🐧 Instrucciones específicas para **Linux Mint**
- 🏛️ Instrucciones para Arch Linux
- 🔧 Troubleshooting y debugging
- ✅ Checklist de validación

## Desinstalación

```bash
systemctl --user stop tisma-agent-polkit
make uninstall
```

## Compatibilidad

- ✅ Arch Linux
- ✅ Arch BTW
- ✅ Derivadas de Arch (Manjaro, EndeavourOS, etc.)
- ✅ Cualquier distro con Rust + Vala + GTK4

## Licencia

MIT/Apache-2.0

## Autor

Tisma

## Referencias

- [PolicyKit Documentation](https://www.freedesktop.org/wiki/Software/polkit/)
- [Vala Documentation](https://wiki.gnome.org/Projects/Vala)
- [GTK4 Documentation](https://docs.gtk.org/gtk4/)
- [D-Bus Specification](https://dbus.freedesktop.org/doc/dbus-daemon.1.html)

---

**Nota**: Este es un proyecto en desarrollo. Se recomienda probar en una máquina virtual antes de usar en producción.
