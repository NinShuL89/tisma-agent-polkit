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
            } else {
                println!("Archivos Vala compilados exitosamente");
            }
        }
        Err(e) => {
            eprintln!("No se pudo ejecutar valac: {}. Asegúrate de tener valac instalado.", e);
            eprintln!("En Arch Linux: sudo pacman -S vala");
        }
    }

    // Obtener información de GTK4 con pkg-config
    let gtk_config = pkg_config::probe_library("gtk4").ok();
    let gio_config = pkg_config::probe_library("gio-2.0").ok();

    if let Some(lib) = gtk_config {
        for path in lib.link_paths {
            println!("cargo:rustc-link-search=native={}", path.display());
        }
        for lib in lib.libs {
            println!("cargo:rustc-link-lib={}", lib);
        }
    }

    if let Some(lib) = gio_config {
        for path in lib.link_paths {
            println!("cargo:rustc-link-search=native={}", path.display());
        }
    }
}
