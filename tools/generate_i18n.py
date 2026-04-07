#!/usr/bin/env python3
"""
Script para generar archivos de traducción base
Uso: python3 tools/generate_i18n.py <código_idioma> <nombre_idioma>

Ejemplo: python3 tools/generate_i18n.py it "Italian"
"""

import json
import sys
from pathlib import Path

# Template base con todas las claves
BASE_TEMPLATE = {
    "app": {
        "name": "Tisma Polkit Agent",
        "description": "[TRANSLATE: PolicyKit authentication agent for Arch Linux]"
    },
    "auth_dialog": {
        "title": "[TRANSLATE: Authentication Required]",
        "action_label": "[TRANSLATE: Action]",
        "message_label": "[TRANSLATE: Please enter your password to continue]",
        "password_placeholder": "[TRANSLATE: Password]",
        "username_label": "[TRANSLATE: Username]",
        "remember_password": "[TRANSLATE: Remember password for this session]",
        "authenticate_button": "[TRANSLATE: Authenticate]",
        "cancel_button": "[TRANSLATE: Cancel]"
    },
    "errors": {
        "auth_failed": "[TRANSLATE: Authentication failed]",
        "invalid_password": "[TRANSLATE: Invalid password]",
        "timeout": "[TRANSLATE: Authentication timeout]",
        "dbus_error": "[TRANSLATE: D-Bus connection error]",
        "permission_denied": "[TRANSLATE: Permission denied]"
    },
    "messages": {
        "starting": "[TRANSLATE: Starting Tisma Polkit Agent...]",
        "registered": "[TRANSLATE: Agent registered with PolicyKit]",
        "waiting_for_auth": "[TRANSLATE: Waiting for authentication request]",
        "auth_success": "[TRANSLATE: Authentication successful]",
        "auth_cancelled": "[TRANSLATE: Authentication cancelled by user]",
        "closing": "[TRANSLATE: Closing agent]"
    }
}

def main():
    if len(sys.argv) < 3:
        print("Uso: python3 generate_i18n.py <código_idioma> <nombre_idioma>")
        print("Ejemplo: python3 generate_i18n.py it Italian")
        sys.exit(1)
    
    lang_code = sys.argv[1]
    lang_name = sys.argv[2]
    
    # Validar código de idioma (máx 2 caracteres)
    if len(lang_code) != 2 or not lang_code.isalpha():
        print(f"Error: código de idioma inválido: {lang_code}")
        print("Debe ser un código ISO 639-1 válido (2 letras minúsculas)")
        sys.exit(1)
    
    lang_code = lang_code.lower()
    
    # Crear archivo
    output_file = Path(f"i18n/{lang_code}.json")
    
    # Verificar si ya existe
    if output_file.exists():
        print(f"Advertencia: {output_file} ya existe")
        response = input("¿Sobrescribir? (s/n): ")
        if response.lower() != 's':
            print("Cancelado")
            sys.exit(0)
    
    # Crear directorio si no existe
    output_file.parent.mkdir(parents=True, exist_ok=True)
    
    # Escribir archivo
    with open(output_file, 'w', encoding='utf-8') as f:
        json.dump(BASE_TEMPLATE, f, indent=2, ensure_ascii=False)
    
    print(f"✓ Archivo creado: {output_file}")
    print(f"\nProximos pasos:")
    print(f"1. Editar {output_file} y traducir all las cadenas")
    print(f"2. Agregar '{lang_name}' al enum Language en src/i18n.rs")
    print(f"3. Agregar el match case en Language::code() y Language::name()")
    print(f"4. Ejecutar: cargo build")
    print(f"5. Probar: LANG={lang_code}_XX.UTF-8 cargo run")
    
    return 0

if __name__ == '__main__':
    sys.exit(main())
