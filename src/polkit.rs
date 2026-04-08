use dbus::blocking::Connection;
use dbus::Message;
use std::time::Duration;
use anyhow::Result;

/// Estructura principal del agente Polkit
pub struct PolkitAgent {
    dbus_connection: Connection,
}

impl PolkitAgent {
    /// Crea una nueva instancia del agente polkit
    pub async fn new() -> Result<Self> {
        // Usar spawn_blocking para operación bloqueante (D-Bus)
        let dbus_connection = tokio::task::spawn_blocking(|| {
            Connection::new_session()
        })
        .await
        .map_err(|e| anyhow::anyhow!("Tokio task error: {}", e))??;

        log::info!("Conexión D-Bus establecida");

        let agent = PolkitAgent { dbus_connection };
        agent.register_agent().await?;

        Ok(agent)
    }

    /// Registra este proceso como agente de autorización polkit
    async fn register_agent(&self) -> Result<()> {
        log::info!("Registrando como agente de autorización polkit...");

        // Obtener el PID del proceso
        let pid = std::process::id();
        
        // Crear mensaje D-Bus para RegisterAuthenticationAgent
        let msg = Message::new_method_call(
            "org.freedesktop.PolicyKit1",
            "/org/freedesktop/PolicyKit1/Authority",
            "org.freedesktop.PolicyKit1.Authority",
            "RegisterAuthenticationAgent",
        )?
        .append2("unix-process", dbus::arg::Struct((
            dbus::arg::Variant(pid),
            None::<&str>,
        )));

        match self.dbus_connection.send(msg) {
            Ok(_) => {
                log::info!("Agente registrado exitosamente (PID: {})", pid);
                Ok(())
            }
            Err(e) => {
                log::error!("Error registrando agente: {}", e);
                Err(anyhow::anyhow!("No se pudo registrar agente PolicyKit: {}", e))
            }
        }
    }

    /// Procesa una solicitud de autorización
    pub async fn handle_auth_request(
        &self,
        action_id: &str,
        message: &str,
        identity: &str,
    ) -> Result<bool> {
        log::info!(
            "Solicitud de autorización: action_id={}, identity={}",
            action_id,
            identity
        );

        // Mostrar diálogo GTK con el mensaje
        // Este método será invocado desde el manejador D-Bus
        // y coordinará con el módulo ui para mostrar el diálogo
        match crate::ui::show_auth_dialog(message, identity).await {
            Ok(authorized) => {
                if authorized {
                    log::info!("Autorización concedida para: {}", action_id);
                } else {
                    log::warn!("Autorización rechazada para: {}", action_id);
                }
                Ok(authorized)
            }
            Err(e) => {
                log::error!("Error mostrando diálogo de autorización: {}", e);
                Ok(false) // Negar por defecto si hay error
            }
        }
    }

    /// Cancela una solicitud de autenticación
    pub async fn cancel_auth_request(&self, cookie: &str) -> Result<()> {
        log::info!("Cancelando solicitud: {}", cookie);
        Ok(())
    }
}
