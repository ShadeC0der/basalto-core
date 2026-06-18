use serde::Deserialize; // Convierte un formato (TOML, JSON, etc) a Rust

#[allow(dead_code)] // Silencia el warning de campos sin usar hasta que config se active
#[derive(Deserialize)]
pub struct Library {
    pub url: String,
    pub branch: String,
}

#[allow(dead_code)] // Silencia el warning de campos sin usar hasta que config se active
#[derive(Deserialize)]
pub struct Config {
    pub library: Library,
}

pub fn read_config() -> Config {
    /* Resumen read_config()
     * Obtiene la ruta al HOME
     * Construye la ruta a ./basalto/config.toml
     * Lee la configuración de config.toml
     * Convierte el .toml al struct de Rust
     */

    let home = std::env::var("HOME").unwrap();
    let route = home + "/.basalto/config.toml";
    let text = std::fs::read_to_string(route).unwrap();
    toml::from_str(&text).unwrap()
}
