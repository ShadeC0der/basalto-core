use serde::Deserialize;

#[derive(Deserialize)]
pub struct Library {
    pub url: String,
    pub branch: String,
}

#[derive(Deserialize)]
pub struct Config {
    pub library: Library,
}

pub fn read_config() -> Config {
    // Obtener HOME
    let home = std::env::var("HOME").unwrap();

    // Construir ruta
    let ruta = home + "/.basalto/config.toml";

    // Leer el archivo
    let texto = std::fs::read_to_string(ruta).unwrap();

    // Convertir y devolver
    toml::from_str(&texto).unwrap()
}
