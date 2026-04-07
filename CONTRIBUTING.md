# Contribuyendo a Tisma Polkit Agent

¡Gracias por tu interés en contribuir! 

## Cómo Contribuir

### 1. Reporte de Bugs
- Crea un issue describiendo el problema
- Incluye logs del error
- Especifica tu distro y versión de Arch
- Describe pasos para reproducir

### 2. Nuevas Características
- Abre un issue para discutir primero
- Sigue el estilo de código existente
- Asegúrate de que tus cambios compilen sin warnings
- Agrega tests si es posible

### 3. Mejoras de Documentación
- Edita README.md, EXAMPLES.md, etc.
- Correcciones de typos
- Ejemplos de uso

## Proceso de Desarrollo

```bash
# Clonar
git clone https://github.com/yourusername/tisma-agent-polkit.git
cd tisma-agent-polkit

# Crear rama
git checkout -b feature/tu-feature

# Compilar y probar
cargo build
cargo check
cargo test

# Hacer commit
git commit -am "Add your feature"

# Push
git push origin feature/tu-feature

# Crear Pull Request
```

## Estándares de Código

### Rust
- Usar `rustfmt` para formateo
- Usar `clippy` para linting
- Documentar funciones públicas
- Agregar ejemplos en docstrings

```bash
cargo fmt
cargo clippy -- -D warnings
```

### Vala
- Seguir convenciones de GNOME
- Comentarios claros
- Código limpio y simple

## Estructura de Commits

```
fix: arreglar bug en polkit handler
feat: agregar soporte para biometría  
docs: actualizar README
style: formatear código
test: agregar tests para autenticación
```

## Áreas de Oportunidad

- [ ] Soporte TOTP/2FA
- [ ] Caché inteligente de sesiones
- [ ] Integración con systemd-ready
- [ ] Tests más completos
- [ ] Documentación en español
- [ ] Traducciones de UI

## Preguntas

Si tienes preguntas, abre una discussion en el repositorio.

---

¡Cada contribución es valiosa!
