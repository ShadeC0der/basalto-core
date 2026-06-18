pub fn ensure(name: &str, source: &str, so_path: &str, branch: &str) {
    /* Resumen de ensure(name, source, so_path, branch)
     * Se obtiene la ruta de HOME
     * Se construye la ruta hacia el cache del plugin
     * Si git no esta instalado imprime un aviso
     * Si cargo no está instalado imprime un aviso
     * Si el repo del plugin no existe, lo clona con git en la ruta del cache
     * Si el .so no existe, lo compila con cargo en modo optimizado
     */

    let home = dirs::home_dir().unwrap();
    let home = home.to_str().unwrap();
    let plugin_dir = format!("{}/.basalto/cache/plugins/{}", home, name);

    if std::process::Command::new("git")
        .arg("--version")
        .output()
        .is_err()
    {
        println!("Error: git no está instalado.");
        std::process::exit(1);
    }

    if std::process::Command::new("cargo")
        .arg("--version")
        .output()
        .is_err()
    {
        println!("Error: cargo no está instalado.");
        std::process::exit(1);
    }

    if !std::path::Path::new(&plugin_dir).exists() {
        std::process::Command::new("git")
            .args(["clone", "-b", branch, source, &plugin_dir])
            .status()
            .unwrap();
    }

    if !std::path::Path::new(so_path).exists() {
        std::process::Command::new("cargo")
            .args(["build", "--release"])
            .current_dir(&plugin_dir)
            .status()
            .unwrap();
    }
}
