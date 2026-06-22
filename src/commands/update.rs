use crate::plugins;
use crate::config;

pub fn run(_args: &[&str]) {
    /* Resumen de run(args)
     * Lee todos los plugins declarados
     * Para cada plugin hace git pull y recompila el .so
     * Clona o actualiza el core en ~/.basalto/cache/core/
     * Si la versión del core cambió reinstala el binario
     */

    let plugins = plugins::read_plugins();
    let home = dirs::home_dir().unwrap();
    let home = home.to_str().unwrap();

    for plugin in &plugins {
        let name = plugin
            .source
            .split('/')
            .next_back()
            .unwrap()
            .trim_end_matches(".git");

        let plugin_dir = format!("{}/.basalto/cache/plugins/{}", home, name);

        if !std::path::Path::new(&plugin_dir).exists() {
            println!("Plugin '{}' no esta en cache, omitiendo.", name);
            continue;
        }

        println!("Actualizando {}...", name);

        std::process::Command::new("git")
            .args(["pull"])
            .current_dir(&plugin_dir)
            .status()
            .unwrap();

        std::process::Command::new("cargo")
            .args(["build", "--release"])
            .current_dir(&plugin_dir)
            .status()
            .unwrap();

        println!("{} actualizado.", name);
    }

    update_core(home);
}

fn update_core(home: &str) {
    /* Resumen de update_core(home)
     * Lee la URL del core desde config.toml
     * Clona el repo si no existe en ~/.basalto/cache/core/
     * Hace git pull si ya existe
     * Compara la versión del cache con la instalada
     * Si hay diferencia reinstala el binario con cargo install
     */

    let core_conf = config::read_core();
    let core_dir = format!("{}/.basalto/cache/core", home);

    if !std::path::Path::new(&core_dir).exists() {
        println!("Clonando basalto-core...");
        std::process::Command::new("git")
            .args(["clone", &core_conf.source, &core_dir])
            .status()
            .unwrap();
    } else {
        println!("Actualizando basalto-core...");
        std::process::Command::new("git")
            .args(["pull"])
            .current_dir(&core_dir)
            .status()
            .unwrap();
    }

    let remote_version = std::fs::read_to_string(format!("{}/Cargo.toml", core_dir))
        .ok()
        .and_then(|content| {
            content
                .lines()
                .find(|l| l.starts_with("version"))
                .and_then(|l| l.split('"').nth(1))
                .map(|v| v.to_string())
        });

    match remote_version {
        Some(v) if v != env!("CARGO_PKG_VERSION") => {
            println!("Nueva version disponible: v{} -> v{}", env!("CARGO_PKG_VERSION"), v);
            println!("Instalando...");
            std::process::Command::new("cargo")
                .args(["install", "--path", &core_dir])
                .status()
                .unwrap();
            println!("basalto-core actualizado a v{}.", v);
        }
        _ => {
            println!("basalto-core ya esta al dia.");
        }
    }
}
