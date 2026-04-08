pub mod i18n;

#[cfg(test)]
mod tests {
    use super::i18n::{I18n, Language};

    #[test]
    fn test_language_detection() {
        // Test para detección de idioma
        let lang = Language::detect();
        assert_ne!(lang.code(), ""); // Debe retornar código válido
    }

    #[test]
    fn test_language_codes() {
        assert_eq!(Language::Spanish.code(), "es");
        assert_eq!(Language::English.code(), "en");
        assert_eq!(Language::French.code(), "fr");
        assert_eq!(Language::German.code(), "de");
        assert_eq!(Language::Portuguese.code(), "pt");
        assert_eq!(Language::Catalan.code(), "ca");
    }

    #[test]
    fn test_language_names() {
        assert_eq!(Language::Spanish.name(), "Spanish");
        assert_eq!(Language::English.name(), "English");
        assert_eq!(Language::French.name(), "French");
        assert_eq!(Language::German.name(), "German");
        assert_eq!(Language::Portuguese.name(), "Portuguese");
        assert_eq!(Language::Catalan.name(), "Catalan");
    }

    #[test]
    fn test_i18n_initialization() {
        let i18n = I18n::new();
        // Verificar que se inicializa sin errores
        let translation = i18n.translate("messages.starting");
        assert!(!translation.is_empty()); // Debe retornar alguna traducción
    }

    #[test]
    fn test_i18n_fallback_to_english() {
        let i18n = I18n::new();
        // Probar fallback en clave inexistente
        let missing = i18n.translate("nonexistent.key");
        assert!(!missing.is_empty()); // Debe retornar algo (fallback)
    }

    #[test]
    fn test_language_equality() {
        assert_eq!(Language::Spanish, Language::Spanish);
        assert_ne!(Language::Spanish, Language::English);
    }
}
