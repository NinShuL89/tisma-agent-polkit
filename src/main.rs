use std::process;

mod polkit;
mod ui;

use tisma_agent_polkit::i18n::{I18n, Language};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();

    // Inicializar i18n con el idioma del sistema
    let mut i18n = I18n::new();
    
    log::info!("{}", i18n.translate("messages.starting"));

    // Crear el servicio polkit
    match polkit::PolkitAgent::new().await {
        Ok(polkit_service) => {
            log::info!("Agente PolicyKit creado exitosamente");
            
            // Iniciar la aplicación GTK con Vala UI
            if let Err(e) = ui::start_gtk_app(polkit_service).await {
                log::error!("Error en GTK: {}", e);
                log::error!("{}", i18n.translate("errors.dbus_error"));
                eprintln!("Error: {}", e);
                process::exit(1);
            }
        }
        Err(e) => {
            log::error!("No se pudo crear el agente PolicyKit: {}", e);
            log::error!("{}", i18n.translate("errors.dbus_error"));
            eprintln!("Error fatal: {}", e);
            process::exit(1);
        }
    }

    Ok(())
}
