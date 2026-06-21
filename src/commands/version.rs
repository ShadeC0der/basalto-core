use crate::plugins::PluginConf;

pub fn run(plugins: &[PluginConf], args: &[&str]) {
    /* Resumen de run(plugins, args)
     * Imprime la versión del core
     * Detecta los flags --active e --inactive en los argumentos
     * Filtra la lista de plugins según el flag recibido (o muestra todos si no hay flag)
     * Para cada plugin imprime nombre, versión leída del Cargo.toml en cache, y estado
     * Usa caracteres ├── y └── para el último elemento, imitando el comando tree
     */

    println!("basalto-core v{}", env!("CARGO_PKG_VERSION"));

    let show_active = args.contains(&"--active");
    let show_inactive = args.contains(&"--inactive");

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

        let prefix = if i + 1 == total {
            "└──"
        } else {
            "├──"
        };
        let status = if p.enabled { "activo" } else { "inactivo" };
        let version = read_plugin_version(name);
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
}
