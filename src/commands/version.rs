use crate::plugins::PluginConf;
use console::style;
use indicatif::{ProgressBar, ProgressStyle};
use std::time::Duration;

pub fn run(plugins: &[PluginConf], args: &[&str]) {
    /* Resumen de run(plugins, args)
     * Imprime la versión del core con estado de actualización si se usa --check
     * Detecta los flags --active, --inactive y --check en los argumentos
     * Filtra la lista de plugins según el flag recibido (o muestra todos si no hay flag)
     * Con --check muestra un spinner mientras consulta el remoto y luego imprime el árbol
     * Usa caracteres ├── y └── para el último elemento, imitando el comando tree
     */

    let check = args.contains(&"--check");
    let show_active = args.contains(&"--active");
    let show_inactive = args.contains(&"--inactive");

    let core_version = env!("CARGO_PKG_VERSION");

    let filtered: Vec<&PluginConf> = plugins
        .iter()
        .filter(|p| {
            if show_active { return p.enabled; }
            if show_inactive { return !p.enabled; }
            true
        })
        .collect();

    if check {
        let pb = spinner();
        pb.set_message("Verificando actualizaciones...");

        let core_update = check_remote_version("");
        let plugin_updates: Vec<Option<String>> = filtered
            .iter()
            .map(|p| {
                let name = p.source.split('/').next_back().unwrap().trim_end_matches(".git");
                needs_update(name)
            })
            .collect();

        pb.finish_and_clear();

        match core_update {
            Some(ref nueva) => println!(
                "basalto-core {}",
                style(format!("v{} → v{}", core_version, nueva)).yellow()
            ),
            None => println!("basalto-core {}", style(format!("v{}", core_version)).cyan()),
        }

        let total = filtered.len();
        for (i, (p, update)) in filtered.iter().zip(plugin_updates.iter()).enumerate() {
            let name = p.source.split('/').next_back().unwrap().trim_end_matches(".git");
            let prefix = style(if i + 1 == total { "└──" } else { "├──" }).dim();
            let version = read_plugin_version(name);
            let info = match update {
                Some(nueva) => format!(
                    "{} {}",
                    style(format!("v{}", version)).cyan(),
                    style(format!("→ v{}", nueva)).yellow()
                ),
                None if !p.enabled => format!(
                    "{} {}",
                    style(format!("v{}", version)).cyan(),
                    style("[inactivo]").dim()
                ),
                None => style(format!("v{}", version)).cyan().to_string(),
            };
            println!("{} {} {}", prefix, name, info);
        }
    } else {
        println!("basalto-core {}", style(format!("v{}", core_version)).cyan());
        let total = filtered.len();
        for (i, p) in filtered.iter().enumerate() {
            let name = p.source.split('/').next_back().unwrap().trim_end_matches(".git");
            let prefix = style(if i + 1 == total { "└──" } else { "├──" }).dim();
            let version = read_plugin_version(name);
            let info = if p.enabled {
                style(format!("v{}", version)).cyan().to_string()
            } else {
                format!(
                    "{} {}",
                    style(format!("v{}", version)).cyan(),
                    style("[inactivo]").dim()
                )
            };
            println!("{} {} {}", prefix, name, info);
        }
    }

    fn read_plugin_version(name: &str) -> String {
        /* Resumen de read_plugin_version(name)
         * Construye la ruta al Cargo.toml del plugin en ~/.basalto/cache/plugins/{name}/
         * Lee el archivo y busca la línea que empieza con "version"
         * Extrae el valor entre comillas y lo retorna
         * Si el archivo no existe o no encuentra la versión, retorna "?"
         */

        let home = dirs::home_dir().unwrap();
        let path = format!(
            "{}/.basalto/cache/plugins/{}/Cargo.toml",
            home.to_str().unwrap(),
            name
        );

        std::fs::read_to_string(path)
            .ok()
            .and_then(|content| {
                content
                    .lines()
                    .find(|l| l.starts_with("version"))
                    .and_then(|l| l.split('"').nth(1))
                    .map(|v| v.to_string())
            })
            .unwrap_or_else(|| "?".to_string())
    }

    fn needs_update(name: &str) -> Option<String> {
        /* Resumen de needs_update(name)
         * Construye la ruta al cache del plugin
         * Hace git fetch para traer el estado del remoto
         * Lee el Cargo.toml remoto para obtener la versión disponible
         * Retorna Some(version) si hay actualización, None si está al día
         */

        let home = dirs::home_dir().unwrap();
        let plugin_dir = format!(
            "{}/.basalto/cache/plugins/{}",
            home.to_str().unwrap(),
            name
        );

        if !std::path::Path::new(&plugin_dir).exists() {
            return None;
        }

        std::process::Command::new("git")
            .args(["fetch"])
            .current_dir(&plugin_dir)
            .output()
            .ok();

        let count = std::process::Command::new("git")
            .args(["rev-list", "HEAD..@{u}", "--count"])
            .current_dir(&plugin_dir)
            .output();

        let behind = match count {
            Ok(o) => String::from_utf8_lossy(&o.stdout).trim().to_string(),
            Err(_) => return None,
        };

        if behind == "0" {
            return None;
        }

        std::process::Command::new("git")
            .args(["show", "@{u}:Cargo.toml"])
            .current_dir(&plugin_dir)
            .output()
            .ok()
            .and_then(|o| {
                String::from_utf8(o.stdout).ok().and_then(|content| {
                    content
                        .lines()
                        .find(|l| l.starts_with("version"))
                        .and_then(|l| l.split('"').nth(1))
                        .map(|v| v.to_string())
                })
            })
    }

    fn spinner() -> ProgressBar {
        let pb = ProgressBar::new_spinner();
        pb.set_style(
            ProgressStyle::with_template("  {spinner:.cyan} {msg}")
                .unwrap()
                .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏", ""]),
        );
        pb.enable_steady_tick(Duration::from_millis(80));
        pb
    }

    fn check_remote_version(_source: &str) -> Option<String> {
        /* Resumen de check_remote_version
         * Usa el cache local en ~/.basalto/cache/core/ (creado por basalto update)
         * Hace git fetch y compara commits con @{u}
         * Si hay commits nuevos lee el Cargo.toml remoto para obtener la versión
         * Retorna Some(version) si hay actualización, None si está al día o no hay cache
         */

        let home = dirs::home_dir()?;
        let core_dir = format!("{}/.basalto/cache/core", home.to_str()?);

        if !std::path::Path::new(&core_dir).exists() {
            return None;
        }

        std::process::Command::new("git")
            .args(["fetch"])
            .current_dir(&core_dir)
            .output()
            .ok();

        let count = std::process::Command::new("git")
            .args(["rev-list", "HEAD..@{u}", "--count"])
            .current_dir(&core_dir)
            .output()
            .ok()?;

        let behind = String::from_utf8_lossy(&count.stdout).trim().to_string();

        if behind == "0" {
            return None;
        }

        std::process::Command::new("git")
            .args(["show", "@{u}:Cargo.toml"])
            .current_dir(&core_dir)
            .output()
            .ok()
            .and_then(|o| {
                String::from_utf8(o.stdout).ok().and_then(|content| {
                    content
                        .lines()
                        .find(|l| l.starts_with("version"))
                        .and_then(|l| l.split('"').nth(1))
                        .map(|v| v.to_string())
                })
            })
    }
}
