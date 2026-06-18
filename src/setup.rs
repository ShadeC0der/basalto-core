pub fn run() {
    /* Resumen de run()
     * Obtiene la ruta al HOME
     * Crea la estructura de carpetas necesaria para que el ecosistema de basalto funcione
     * Crea .basalto/config.toml
     * Si no existe el config.toml lo crea con una plantilla
     */

    let home = dirs::home_dir().unwrap();
    let home = home.to_str().unwrap();

    std::fs::create_dir_all(format!("{}/.basalto/plugins", home)).unwrap();
    std::fs::create_dir_all(format!("{}/.basalto/cache/plugins", home)).unwrap();
    std::fs::create_dir_all(format!("{}/.basalto/cache/library", home)).unwrap();

    let config_path = format!("{}/.basalto/config.toml", home);

    if !std::path::Path::new(&config_path).exists() {
        std::fs::write(&config_path, "[library]\nurl = \"\"\nbranch = \"main\"\n").unwrap();
    }
}
