use std::process::Command;
use std::env;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    // Compilar archivos Vala a C
    let valac_output = Command::new("valac")
        .arg("--ccode")
        .arg("--pkg")
        .arg("gtk4")
        .arg("--pkg")
        .arg("gio-2.0")
        .arg(format!("{}/vala/polkit_dialog.vala", manifest_dir))
        .arg("-d")
        .arg(&out_dir)
        .output();

    match valac_output {
        Ok(output) => {
            if !output.status.success() {
                eprintln!("Error compilando Vala:");
                eprintln!("{}", String::from_utf8_lossy(&output.stderr));
                panic!("❌ La compilación de archivos Vala falló. Verifica la sintaxis Vala.");
            } else {
                println!("cargo:warning=✓ Archivos Vala compilados exitosamente");
            }
        }
        Err(e) => {
            panic!(
                "❌ No se pudo ejecutar valac: {}\n\
                 Asegúrate de tener valac instalado:\n\
                 En Arch Linux: sudo pacman -S vala\n\
                 En Debian/Ubuntu: sudo apt-get install valac",
                e
            );
        }
    }

    // Obtener información de GTK4 con pkg-config
    let gtk_config = pkg_config::probe_library("gtk4");
    let gio_config = pkg_config::probe_library("gio-2.0");

    if let Ok(lib) = gtk_config {
        for path in lib.link_paths {
            println!("cargo:rustc-link-search=native={}", path.display());
        }
        for lib in lib.libs {
            println!("cargo:rustc-link-lib={}", lib);
        }
    } else {
        panic!(
            "❌ No se encontró gtk4 configurado en pkg-config.\n\
             En Arch Linux: sudo pacman -S gtk4"
        );
    }

    if let Ok(lib) = gio_config {
        for path in lib.link_paths {
            println!("cargo:rustc-link-search=native={}", path.display());
        }
        for lib in lib.libs {
            println!("cargo:rustc-link-lib={}", lib);
        }
    } else {
        panic!(
            "❌ No se encontró gio-2.0 configurado en pkg-config.\n\
             En Arch Linux: sudo pacman -S glib2"
        );
    }
}

