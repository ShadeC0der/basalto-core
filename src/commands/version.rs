use crate::plugins::PluginConf;
use crate::config;

pub fn run(plugins: &[PluginConf], args: &[&str]) {
    /* Resumen de run(plugins, args)
     * Imprime la versión del core con estado de actualización si se usa --check
     * Detecta los flags --active, --inactive y --check en los argumentos
     * Filtra la lista de plugins según el flag recibido (o muestra todos si no hay flag)
     * Para cada plugin imprime nombre, versión leída del Cargo.toml en cache, y estado
     * Con --check muestra la versión remota disponible tanto del core como de los plugins
     * Usa caracteres ├── y └── para el último elemento, imitando el comando tree
     */

    let check = args.contains(&"--check");
    let show_active = args.contains(&"--active");
    let show_inactive = args.contains(&"--inactive");

    let core_version = env!("CARGO_PKG_VERSION");

    if check {
        let core_conf = config::read_core();
        match check_remote_version(&core_conf.source) {
            Some(nueva) => println!(
                "basalto-core v{} -> v{} (actualizar)",
                core_version, nueva
            ),
            None => println!("basalto-core v{}", core_version),
        }
    } else {
        println!("basalto-core v{}", core_version);
    }

    let filtered: Vec<&PluginConf> = plugins
        .iter()
        .filter(|p| {
            if show_active {
                return p.enabled;
            }
            if show_inactive {
                return !p.enabled;
            }
            true
        })
        .collect();

    let total = filtered.len();

    for (i, p) in filtered.iter().enumerate() {
        let name = p
            .source
            .split('/')
            .next_back()
            .unwrap()
            .trim_end_matches(".git");

        let prefix = if i + 1 == total { "└──" } else { "├──" };
        let version = read_plugin_version(name);

        let status = if check {
            match needs_update(name) {
                Some(nueva) => format!("-> v{} (actualizar)", nueva),
                None => if p.enabled { "activo".to_string() } else { "inactivo".to_string() },
            }
        } else if p.enabled {
            "activo".to_string()
        } else {
            "inactivo".to_string()
        };

        println!("{} {} v{} ({})", prefix, name, version, status);
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

    fn check_remote_version(source: &str) -> Option<String> {
        /* Resumen de check_remote_version(source)
         * Usa git ls-remote para obtener el Cargo.toml del remoto sin clonar
         * Extrae la versión del contenido remoto
         * Retorna Some(version) si es distinta a la local, None si coincide
         */

        let output = std::process::Command::new("git")
            .args(["ls-remote", source, "HEAD"])
            .output()
            .ok()?;

        let sha = String::from_utf8(output.stdout)
            .ok()?
            .split_whitespace()
            .next()?
            .to_string();

        let cargo = std::process::Command::new("git")
            .args(["archive", &format!("--remote={}", source), &sha, "Cargo.toml"])
            .output()
            .ok()?;

        let content = String::from_utf8(cargo.stdout).ok()?;
        let remote_version = content
            .lines()
            .find(|l| l.starts_with("version"))
            .and_then(|l| l.split('"').nth(1))
            .map(|v| v.to_string())?;

        if remote_version != env!("CARGO_PKG_VERSION") {
            Some(remote_version)
        } else {
            None
        }
    }
}
