use serde::Deserialize; // Convierte un formato (TOML, JSON, etc) a Rust

#[derive(Deserialize)]
pub struct PluginConf {
    pub source: String,
    pub branch: String,
}

pub fn read_plugins() -> Vec<PluginConf> {
    /* Resumen de read_plugins()
     * Obtiene la ruta al HOME
     * Obtiene la ruta hasta ./basalto/plugins/
     * Lee el contenido de la carpeta plugins/
     * Se crea un nuevo vector vacío llamado plugins
     * En el for se toma cada archivo de plugins/
     *  Dentro del for se obtiene la ruta de cada archivo
     *  Con la ruta del archivo se lee el texto plano
     *  Con serde y el struct PluginConf obtenemos source y branch del TOML
     *  y pasan a ser campos de Rust
     *   Y se agregan al vector plugins
     * Y al final retorna el vector de todos los plugins declarados
     */

    let home = dirs::home_dir().unwrap();
    let home = home.to_str().unwrap();
    let route = format!("{}/.basalto/plugins/", home);
    let content = std::fs::read_dir(route).unwrap();
    let mut plugins = Vec::new();

    for input in content {
        let route_file = input.unwrap().path();
        let text = std::fs::read_to_string(route_file).unwrap();
        let plugin: PluginConf = toml::from_str(&text).unwrap();
        plugins.push(plugin);
    }

    plugins
}
