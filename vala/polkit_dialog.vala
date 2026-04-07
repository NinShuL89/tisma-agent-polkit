/* Tisma Polkit Agent - UI en Vala */

using Gtk;
using Gio;

public class PolkitAuthDialog : Gtk.ApplicationWindow {
    private Gtk.Label action_label;
    private Gtk.Label message_label;
    private Gtk.Entry password_entry;
    private Gtk.Button auth_button;
    private Gtk.Button cancel_button;

    public signal void auth_confirmed(string password);
    public signal void auth_cancelled();

    public PolkitAuthDialog(Gtk.Application app) {
        Object(application: app, type: Gtk.WindowType.TOPLEVEL);
        
        this.set_title("Tisma - Autorización Requerida");
        this.set_default_size(400, 250);
        this.set_resizable(false);
        this.set_keep_above(true);
        this.set_type_hint(Gdk.WindowTypeHint.DIALOG);
        
        // Hacer ventana modal
        this.modal = true;
        
        // Crear estructura de la UI
        build_ui();
        
        // Conectar señales
        this.auth_button.clicked.connect(() => {
            string password = this.password_entry.get_text();
            auth_confirmed(password);
            this.destroy();
        });
        
        this.cancel_button.clicked.connect(() => {
            auth_cancelled();
            this.destroy();
        });
    }

    private void build_ui() {
        var main_box = new Gtk.Box(Gtk.Orientation.VERTICAL, 12);
        main_box.margin = 20;
        
        // Título/Acción
        this.action_label = new Gtk.Label("Se requiere autenticación");
        this.action_label.set_halign(Gtk.Align.START);
        var action_font = Pango.FontDescription.from_string("bold 14");
        this.action_label.override_font(action_font);
        main_box.append(this.action_label);
        
        // Mensaje
        this.message_label = new Gtk.Label("Por favor, ingre su contraseña para continuar");
        this.message_label.set_halign(Gtk.Align.START);
        this.message_label.set_wrap(true);
        main_box.append(this.message_label);
        
        // Separador
        var separator = new Gtk.Separator(Gtk.Orientation.HORIZONTAL);
        main_box.append(separator);
        
        // Campo de contraseña
        var password_box = new Gtk.Box(Gtk.Orientation.HORIZONTAL, 10);
        var password_label = new Gtk.Label("Contraseña:");
        password_label.set_halign(Gtk.Align.END);
        password_label.set_hexpand(false);
        password_box.append(password_label);
        
        this.password_entry = new Gtk.Entry();
        this.password_entry.set_visibility(false);
        this.password_entry.set_input_purpose(Gtk.InputPurpose.PASSWORD);
        this.password_entry.set_hexpand(true);
        password_box.append(this.password_entry);
        main_box.append(password_box);
        
        // Botones
        var button_box = new Gtk.Box(Gtk.Orientation.HORIZONTAL, 10);
        button_box.set_halign(Gtk.Align.END);
        
        this.cancel_button = new Gtk.Button.with_label("Cancelar");
        this.cancel_button.set_size_request(100, -1);
        button_box.append(this.cancel_button);
        
        this.auth_button = new Gtk.Button.with_label("Autenticar");
        this.auth_button.set_size_request(100, -1);
        this.auth_button.add_css_class("suggested-action");
        button_box.append(this.auth_button);
        
        main_box.append(button_box);
        
        this.set_child(main_box);
    }

    public void set_action(string action_id, string message) {
        this.action_label.set_label("Acción: %s".printf(action_id));
        this.message_label.set_label(message);
    }

    public string get_password() {
        return this.password_entry.get_text();
    }

    public void clear_password() {
        this.password_entry.set_text("");
    }
}

public class PolkitAgentApp : Gtk.Application {
    public PolkitAgentApp() {
        Object(application_id: "org.tisma.PolkitAgent");
    }

    protected override void activate() {
        // La ventana se creará cuando sea necesaria
        log("Aplicación iniciada");
    }

    public static int main(string[] args) {
        return new PolkitAgentApp().run(args);
    }
}
