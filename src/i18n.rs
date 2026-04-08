//! Módulo de internacionalización (i18n)
//! 
//! Gestiona las traducciones del aplicativo en múltiples idiomas.

use serde_json::{json, Value};
use std::collections::HashMap;
use std::env;
use std::path::Path;
use log::warn;

/// Idiomas soportados
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Language {
    Spanish,   // es
    English,   // en
    French,    // fr
    German,    // de
    Portuguese, // pt
    Catalan,   // ca
}

impl Language {
    /// Obtener código de idioma (ISO 639-1)
    pub fn code(&self) -> &'static str {
        match self {
            Language::Spanish => "es",
            Language::English => "en",
            Language::French => "fr",
            Language::German => "de",
            Language::Portuguese => "pt",
            Language::Catalan => "ca",
        }
    }

    /// Obtener nombre del idioma en inglés
    pub fn name(&self) -> &'static str {
        match self {
            Language::Spanish => "Spanish",
            Language::English => "English",
            Language::French => "French",
            Language::German => "German",
            Language::Portuguese => "Portuguese",
            Language::Catalan => "Catalan",
        }
    }

    /// Detectar idioma del sistema
    pub fn detect() -> Self {
        let lang = env::var("LANG")
            .or_else(|_| env::var("LC_ALL"))
            .unwrap_or_default()
            .to_lowercase();

        if lang.starts_with("es") {
            Language::Spanish
        } else if lang.starts_with("fr") {
            Language::French
        } else if lang.starts_with("de") {
            Language::German
        } else if lang.starts_with("pt") {
            Language::Portuguese
        } else if lang.starts_with("ca") {
            Language::Catalan
        } else {
            Language::English // Default
        }
    }
}

/// Gestor de traducciones
pub struct I18n {
    language: Language,
    translations: HashMap<Language, Value>,
}

impl I18n {
    /// Crear una nueva instancia con el idioma del sistema
    pub fn new() -> Self {
        Self::with_language(Language::detect())
    }

    /// Crear una instancia con un idioma específico
    pub fn with_language(language: Language) -> Self {
        log::info!("Inicializando i18n con idioma: {}", language.name());

        let mut translations = HashMap::new();
        
        // Cargar todas las traducciones disponibles
        let languages = vec![
            Language::Spanish,
            Language::English,
            Language::French,
            Language::German,
            Language::Portuguese,
            Language::Catalan,
        ];

        for lang in languages {
            if let Ok(trans) = Self::load_translation_file(lang) {
                translations.insert(lang, trans);
            }
        }

        Self { language, translations }
    }

    /// Cargar archivo de traducción
    fn load_translation_file(lang: Language) -> Result<Value, Box<dyn std::error::Error>> {
        // Intentar cargar desde el sistema de archivos primero (modo desarrollo)
        let filename = format!("i18n/{}.json", lang.code());
        
        if let Ok(content) = std::fs::read_to_string(&filename) {
            return Ok(serde_json::from_str(&content)?);
        }
        
        // Fallback: retornar JSON vacío (las claves de idioma deberían ser embebidas)
        // En producción, aquí iría include_str!() para embeber las traducciones
        log::warn!("Archivo de traducción {} no encontrado. Usando fallback.", filename);
        
        Ok(json!({
            "messages": {},
            "errors": {}
        }))
    }

    /// Cambiar idioma
    pub fn set_language(&mut self, language: Language) {
        log::info!("Cambiando idioma a: {}", language.name());
        self.language = language;
    }

    /// Obtener idioma actual
    pub fn get_language(&self) -> Language {
        self.language
    }

    /// Traducir una clave
    pub fn translate(&self, key: &str) -> String {
        self.translate_with_language(self.language, key)
    }

    /// Traducir una clave con un idioma específico
    pub fn translate_with_language(&self, language: Language, key: &str) -> String {
        let parts: Vec<&str> = key.split('.').collect();

        if let Some(trans) = self.translations.get(&language) {
            let mut current = trans;

            for part in parts {
                if let Some(next) = current.get(part) {
                    current = next;
                } else {
                    warn!("Clave de traducción no encontrada: {} ({})", key, language.code());
                    // Intentar con inglés como fallback
                    if language != Language::English {
                        return self.translate_with_language(Language::English, key);
                    }
                    return key.to_string();
                }
            }

            current
                .as_str()
                .map(|s| s.to_string())
                .unwrap_or_else(|| key.to_string())
        } else {
            warn!("Idioma no disponible: {}", language.name());
            key.to_string()
        }
    }

    /// Traducir con variables
    pub fn translate_with_args(&self, key: &str, args: &[(&str, &str)]) -> String {
        let mut translation = self.translate(key);

        for (var, value) in args {
            translation = translation.replace(&format!("{{{}}}", var), value);
        }

        translation
    }

    /// Obtener lista de idiomas disponibles
    pub fn available_languages() -> Vec<(Language, &'static str)> {
        vec![
            (Language::Spanish, "Español"),
            (Language::English, "English"),
            (Language::French, "Français"),
            (Language::German, "Deutsch"),
            (Language::Portuguese, "Português"),
            (Language::Catalan, "Català"),
        ]
    }
}

impl Default for I18n {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_language_detection() {
        let lang = Language::detect();
        assert!(!format!("{:?}", lang).is_empty());
    }

    #[test]
    fn test_language_codes() {
        assert_eq!(Language::Spanish.code(), "es");
        assert_eq!(Language::English.code(), "en");
        assert_eq!(Language::French.code(), "fr");
    }

    #[test]
    fn test_available_languages() {
        let langs = I18n::available_languages();
        assert_eq!(langs.len(), 6);
    }
}
