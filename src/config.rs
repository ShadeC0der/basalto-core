use serde::Deserialize;

#[allow(dead_code)]
#[derive(Deserialize)]
pub struct Library {
    pub url: String,
    pub branch: String,
}

#[allow(dead_code)]
#[derive(Deserialize)]
pub struct Config {
    pub library: Library,
}

pub fn read_config() -> Config {
    // Obtener HOME
    let home = std::env::var("HOME").unwrap();

    // Construir ruta
    let route = home + "/.basalto/config.toml";

    // Leer el archivo
    let text = std::fs::read_to_string(route).unwrap();

    // Convertir y devolver
    toml::from_str(&text).unwrap()
}
