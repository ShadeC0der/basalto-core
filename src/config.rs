use serde::Deserialize;

#[derive(Deserialize)]
pub struct CoreConfig {
    pub source: String,
}

#[derive(Deserialize)]
pub struct TuiConfig {
    pub source: String,
}

#[derive(Deserialize, Clone)]
pub struct LibraryEntry {
    pub name: String,
    pub source: String,
}

#[derive(Deserialize, Default)]
pub struct LibrariesConfig {
    #[serde(default)]
    pub active: String,
    #[serde(default)]
    pub list: Vec<LibraryEntry>,
}

#[derive(Deserialize)]
pub struct Config {
    pub core: CoreConfig,
    #[serde(default)]
    pub tui: Option<TuiConfig>,
    #[serde(default)]
    pub libraries: LibrariesConfig,
}

pub fn read_config() -> Config {
    let home = dirs::home_dir().unwrap();
    let route = format!("{}/.basalto/config.toml", home.to_str().unwrap());
    let text = std::fs::read_to_string(route).unwrap();
    toml::from_str(&text).unwrap()
}

pub fn read_core() -> CoreConfig {
    read_config().core
}
