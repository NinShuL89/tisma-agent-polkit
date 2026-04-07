.PHONY: build release install uninstall clean help i18n-list i18n-new i18n-check

PREFIX ?= /usr/local

help:
	@echo "Tisma Polkit Agent - Targets disponibles:"
	@echo ""
	@echo "Compilación:"
	@echo "  make build        - Compilar en modo debug"
	@echo "  make release      - Compilar en modo release (optimizado)"
	@echo "  make install      - Instalar el agente"
	@echo "  make uninstall    - Desinstalar el agente"
	@echo "  make clean        - Limpiar archivos compilados"
	@echo ""
	@echo "Traducciones (i18n):"
	@echo "  make i18n-list    - Listar idiomas soportados"
	@echo "  make i18n-new     - Crear nueva traducción"
	@echo "  make i18n-check   - Verificar integridad JSON"
	@echo ""
	@echo "Ejemplo crear idioma italiano:"
	@echo "  make i18n-new LANG_CODE=it LANG_NAME='Italian'"

build:
	@echo "Compilando Tisma Polkit Agent (debug)..."
	cargo build

release:
	@echo "Compilando Tisma Polkit Agent (release)..."
	cargo build --release

install: release
	@echo "Instalando Tisma Polkit Agent..."
	install -Dm755 target/release/tisma-agent-polkit $(DESTDIR)$(PREFIX)/bin/tisma-agent-polkit
	install -Dm644 systemd/tisma-agent-polkit.service $(DESTDIR)/usr/lib/systemd/user/tisma-agent-polkit.service
	install -Dm644 dbus/org.tisma.PolkitAgent.xml $(DESTDIR)$(PREFIX)/share/dbus-1/interfaces/org.tisma.PolkitAgent.xml
	install -Dm644 policies/org.tisma.policy $(DESTDIR)$(PREFIX)/share/polkit-1/actions/org.tisma.policy
	@echo "Instalación completada. Ejecutar: systemctl --user enable tisma-agent-polkit"

uninstall:
	@echo "Desinstalando Tisma Polkit Agent..."
	rm -f $(DESTDIR)$(PREFIX)/bin/tisma-agent-polkit
	rm -f $(DESTDIR)/usr/lib/systemd/user/tisma-agent-polkit.service
	rm -f $(DESTDIR)$(PREFIX)/share/dbus-1/interfaces/org.tisma.PolkitAgent.xml
	rm -f $(DESTDIR)$(PREFIX)/share/polkit-1/actions/org.tisma.policy

clean:
	cargo clean
	rm -f vala_tmp/*
	find . -name "*.c" -o -name "*.h" | grep -v src | xargs rm -f

# Targets de internacionalización
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
		exit 1; \
	fi
	python3 tools/generate_i18n.py $(LANG_CODE) $(LANG_NAME)

i18n-check:
	@echo "Verificando integridad de traducciones..."
	@for file in i18n/*.json; do \
		python3 -m json.tool "$$file" > /dev/null && echo "  ✓ $$file" || echo "  ✗ $$file"; \
	done
