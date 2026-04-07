# Traducciones - Tisma Polkit Agent

Este proyecto soporta múltiples idiomas a través de archivos JSON en la carpeta `i18n/`.

## Idiomas Soportados

| Código | Idioma | Archivo | Estado |
|--------|--------|---------|--------|
| `es` | Español | `i18n/es.json` | ✅ Completo |
| `en` | English | `i18n/en.json` | ✅ Completo |
| `fr` | Français | `i18n/fr.json` | ✅ Completo |
| `de` | Deutsch | `i18n/de.json` | ✅ Completo |
| `pt` | Português | `i18n/pt.json` | ✅ Completo |
| `ca` | Català | `i18n/ca.json` | ✅ Completo |

## Estructura de Traducción

Cada archivo contiene una estructura JSON jerárquica:

```json
{
  "app": {
    "name": "...",
    "description": "..."
  },
  "auth_dialog": {
    "title": "...",
    "password_placeholder": "..."
  },
  "errors": {
    "auth_failed": "...",
    "invalid_password": "..."
  },
  "messages": {
    "starting": "...",
    "auth_success": "..."
  }
}
```

## Cómo Usar en Código

### Rust
```rust
use tisma_agent_polkit::i18n::{I18n, Language};

// Crear con idioma del sistema
let i18n = I18n::new();

// Traducir una clave
let message = i18n.translate("messages.starting");

// Cambiar idioma
let mut i18n = I18n::new();
i18n.set_language(Language::Spanish);

// Traducir con variables
let msg = i18n.translate_with_args(
    "welcome",
    &[("name", "José")],
);
```

### Vala
```vala
// Las traducciones en Vala se cargan desde el backend Rust
// Los diálogos deben recibir strings traducidos del motor Rust
```

## Agregar un Nuevo Idioma

1. **Crear archivo** de traducción. Ejemplo: `i18n/it.json` (para italiano)

```json
{
  "app": {
    "name": "Tisma Polkit Agent",
    "description": "..."
  },
  // ... resto de las claves
}
```

2. **Agregar al enum** `Language` en `src/i18n.rs`:

```rust
pub enum Language {
    Spanish,
    English,
    // ... otros idiomas
    Italian,  // Nuevo
}

impl Language {
    pub fn code(&self) -> &'static str {
        match self {
            // ...
            Language::Italian => "it",
        }
    }
    
    pub fn name(&self) -> &'static str {
        match self {
            // ...
            Language::Italian => "Italian",
        }
    }
}
```

3. **Actualizar método `detect()`** en `src/i18n.rs` si es necesario.

4. **Actualizar** `available_languages()`.

5. **Compilar:**
```bash
cargo build
```

## Detección Automática de Idioma

El sistema detecta automáticamente el idioma del usuario basándose en:

1. Variable de entorno `LANG` (ej: `es_ES.UTF-8`)
2. Variable de entorno `LC_ALL`
3. Fallback a inglés si no coincide con ninguno

```bash
# Probar en español
LANG=es_ES.UTF-8 cargo run

# Probar en francés
LANG=fr_FR.UTF-8 cargo run
```

## Estructura de Claves

Usar notación de punto para acceder a claves anidadas:

```
app.name                    → "Tisma Polkit Agent"
auth_dialog.title          → "Se requiere autenticación"
errors.auth_failed         → "Autenticación fallida"
messages.auth_success      → "Autenticación exitosa"
```

## Contribuir Traducciones

Para hacer una traducción:

1. **Clonar** la estructura de `i18n/en.json`
2. **Traducir** cada cadena con precisión
3. **Verificar** caracteres especiales y puntuación
4. **Probar** con `LANG=código cargo run`
5. **Crear PR** con los cambios

### Consejos de Traducción

- Mantener la longitud similar al inglés (evita rebalses en UI)
- Usar términos técnicos consistentes
- Preservar variables entre llaves: `{variable}`
- Mantener puntuación según el idioma

## Variables en Traducciones

Algunas traducciones soportan variables:

```json
"welcome": "Bienvenido, {name}!"
```

En código:
```rust
let msg = i18n.translate_with_args(
    "welcome",
    &[("name", "Juan")],
);
// Resultado: "Bienvenido, Juan!"
```

## Testing de Traducciones

```bash
# Ver todos los idiomas
cargo test i18n -- --nocapture

# Probar un idioma específico
LANG=de_DE.UTF-8 cargo run
```

## Licencia

Las traducciones están bajo la misma licencia del proyecto (MIT/Apache-2.0).

---

¡Gracias por ayudar a que Tisma sea más accesible!
