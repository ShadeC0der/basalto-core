pub fn ensure(name: &str, source: &str, so_path: &str, branch: &str) {
    /* Resumen de ensure(name, source, so_path, branch)
     * Se obtiene la ruta de HOME
     * Se construye la ruta hacia el cache del plugin
     * Si el repo del plugin no existe, lo clona con git en la ruta del cache
     * Si el .so no existe, lo compila con cargo en modo optimizado
     */

    let home = std::env::var("HOME").unwrap();
    let plugin_dir = format!("{}/.basalto/cache/plugins/{}", home, name);

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
