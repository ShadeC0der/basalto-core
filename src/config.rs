use serde::Deserialize;

#[allow(dead_code)]
#[derive(Deserialize)]
pub struct Library {
    pub url: String,
    pub branch: String,
}

#[derive(Deserialize)]
pub struct CoreConfig {
    pub source: String,
}

#[derive(Deserialize)]
pub struct TuiConfig {
    pub source: String,
}

#[allow(dead_code)]
#[derive(Deserialize)]
pub struct Config {
    pub library: Library,
    pub core: CoreConfig,
    #[serde(default)]
    pub tui: Option<TuiConfig>,
}

pub fn read_config() -> Config {
    /* Resumen read_config()
     * Obtiene la ruta al HOME
     * Construye la ruta a ./basalto/config.toml
     * Lee la configuración de config.toml
     * Convierte el .toml al struct de Rust
     */

    let home = dirs::home_dir().unwrap();
    let home = home.to_str().unwrap();
    let route = format!("{}/.basalto/config.toml", home);
    let text = std::fs::read_to_string(route).unwrap();
    toml::from_str(&text).unwrap()
}

pub fn read_core() -> CoreConfig {
    /* Resumen read_core()
     * Lee config.toml y retorna la sección [core]
     */
    read_config().core
}
