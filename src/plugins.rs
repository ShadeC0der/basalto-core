use serde::Deserialize;

#[derive(Deserialize)]
pub struct PluginConf {
    pub source: String,
    pub branch: String,
}

pub fn read_plugins() -> Vec<PluginConf> {
    // Obtener HOME
    let home = std::env::var("HOME").unwrap();

    // Construir ruta a la carpeta
    let route = home + "/.basalto/plugins/";

    // Leer el contenido
    let content = std::fs::read_dir(route).unwrap();

    let mut plugins = Vec::new();

    // Convertir y devolver cada archivo
    for input in content {
        let route_file = input.unwrap().path();
        let text = std::fs::read_to_string(route_file).unwrap();
        let plugin: PluginConf = toml::from_str(&text).unwrap();
        plugins.push(plugin);
    }

    plugins
}
