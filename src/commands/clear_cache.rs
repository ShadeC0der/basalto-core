use indicatif::{ProgressBar, ProgressStyle};
use std::time::Duration;

pub fn run(args: &[&str]) {
    /* Resumen de run(args)
     * Sin flags: muestra lo que se va a borrar y pide confirmacion con --yes
     * Con --yes: borra ~/.basalto/cache/ y recrea la estructura de carpetas vacia
     */

    let home = dirs::home_dir().unwrap();
    let cache_dir = format!("{}/.basalto/cache", home.to_str().unwrap());

    if !std::path::Path::new(&cache_dir).exists() {
        println!("El cache ya esta vacio.");
        return;
    }

    let tamanio = tamanio_dir(&cache_dir);

    if !args.contains(&"--yes") {
        println!("Se borrara: {}", cache_dir);
        println!("Tamanio:    {}", formatear_bytes(tamanio));
        println!("");
        println!("Los plugins se volvera a clonar y compilar la proxima vez que se usen.");
        println!("Para confirmar: basalto clear-cache --yes");
        return;
    }

    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::with_template("  {spinner:.cyan} {msg}")
            .unwrap()
            .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏", ""]),
    );
    pb.enable_steady_tick(Duration::from_millis(80));
    pb.set_message("Limpiando cache...");

    std::fs::remove_dir_all(&cache_dir).unwrap();

    std::fs::create_dir_all(format!("{}/.basalto/cache/plugins", home.to_str().unwrap())).unwrap();
    std::fs::create_dir_all(format!("{}/.basalto/cache/library", home.to_str().unwrap())).unwrap();

    pb.finish_with_message(format!("✓ Cache limpiado  ({} liberados)", formatear_bytes(tamanio)));
}

fn tamanio_dir(path: &str) -> u64 {
    let mut total = 0u64;
    if let Ok(entries) = std::fs::read_dir(path) {
        for entry in entries.flatten() {
            let meta = entry.metadata();
            if let Ok(m) = meta {
                if m.is_dir() {
                    total += tamanio_dir(&entry.path().to_string_lossy());
                } else {
                    total += m.len();
                }
            }
        }
    }
    total
}

fn formatear_bytes(bytes: u64) -> String {
    if bytes >= 1_073_741_824 {
        format!("{:.1} GB", bytes as f64 / 1_073_741_824.0)
    } else if bytes >= 1_048_576 {
        format!("{:.1} MB", bytes as f64 / 1_048_576.0)
    } else if bytes >= 1_024 {
        format!("{:.1} KB", bytes as f64 / 1_024.0)
    } else {
        format!("{} B", bytes)
    }
}
