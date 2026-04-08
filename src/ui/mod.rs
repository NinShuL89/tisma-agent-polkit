use crate::polkit::PolkitAgent;
use anyhow::Result;

pub mod vala_ui;

/// Inicia la aplicación GTK con la interfaz de Vala
pub async fn start_gtk_app(polkit_agent: PolkitAgent) -> Result<()> {
    log::info!("Iniciando interfaz GTK...");

    // Inicializar GTK
    gtk::init()?;
    
    // Configurar el agente PolicyKit
    let _agent_ref = polkit_agent;

    // Crear y mostrar la ventana principal
    build_main_window(&_agent_ref)?;

    // Ejecutar el loop principal GTK
    gtk::main();
    
    Ok(())
}

/// Construcción de la ventana principal
fn build_main_window(polkit_agent: &PolkitAgent) -> Result<()> {
    use gtk::prelude::*;

    let window = gtk::ApplicationWindow::new(&gtk::Application::new(
        Some("org.tisma.PolkitAgent"),
        Default::default(),
    ));
    
    window.set_title(Some("Tisma Polkit Agent"));
    window.set_default_size(400, 200);
    
    let label = gtk::Label::new(Some("Agente de autorización Polkit activo"));
    window.set_child(Some(&label));
    
    window.connect_close_request(|_| {
        gtk::main_quit();
        gtk::glib::Propagation::Stop
    });
    
    window.show();
    
    log::info!("Ventana principal creada");
    Ok(())
}

/// Muestra el diálogo de autorización
pub async fn show_auth_dialog(message: &str, identity: &str) -> Result<bool> {
    log::info!("Mostrando diálogo de autorización para: {}", identity);
    
    // Esta función será implementada con la UI de Vala
    // Por ahora retorna false (negar)
    // TODO: Integrar con interfaz compilada desde Vala
    
    Ok(false)
}
