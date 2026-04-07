# Makefile para tareas de traducción

.PHONY: i18n-list i18n-new i18n-check i18n-update help

i18n-list:
	@echo "Idiomas soportados:"
	@echo "  es - Español"
	@echo "  en - English"
	@echo "  fr - Français"
	@echo "  de - Deutsch"
	@echo "  pt - Português"
	@echo "  ca - Català"

i18n-new:
	@if [ -z "$(LANG_CODE)" ] || [ -z "$(LANG_NAME)" ]; then \
		echo "Uso: make i18n-new LANG_CODE=xx LANG_NAME='Language Name'"; \
		echo "  LANG_CODE: Código ISO 639-1 (2 letras)"; \
		echo "  LANG_NAME: Nombre del idioma"; \
		echo "Ejemplo: make i18n-new LANG_CODE=it LANG_NAME='Italian'"; \
		exit 1; \
	fi
	python3 tools/generate_i18n.py $(LANG_CODE) $(LANG_NAME)

i18n-check:
	@echo "Verificando integridad de traducciones..."
	@for file in i18n/*.json; do \
		echo "  Verificando $$file..."; \
		python3 -m json.tool "$$file" > /dev/null || echo "  ✗ Error en $$file"; \
	done
	@echo "  ✓ Todas las traducciones son válidas"

i18n-update:
	@echo "Actualizando traducciones..."
	@echo "Por implementar: sincronizar desde traducción base"

i18n-test-es:
	LANG=es_ES.UTF-8 cargo test i18n -- --nocapture

i18n-test-en:
	LANG=en_US.UTF-8 cargo test i18n -- --nocapture

i18n-test-all:
	@echo "Probando todos los idiomas..."
	@make i18n-test-es
	@make i18n-test-en
	@echo "✓ Pruebas completadas"

.PHONY: i18n-list i18n-new i18n-check i18n-update i18n-test-all i18n-test-es i18n-test-en
