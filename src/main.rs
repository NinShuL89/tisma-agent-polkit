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
    let polkit_service = polkit::PolkitAgent::new().await?;

    // Iniciar la aplicación GTK con Vala UI
    if let Err(e) = ui::start_gtk_app(polkit_service).await {
        log::error!("{}", i18n.translate("errors.dbus_error"));
        process::exit(1);
    }

    Ok(())
}
