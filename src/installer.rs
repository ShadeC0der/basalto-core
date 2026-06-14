pub fn ensure(name: &str, source: &str, so_path: &str) {
    let home = std::env::var("HOME").unwrap();
    let plugin_dir = format!("{}/.basalto/cache/plugins/{}", home, name);

    if !std::path::Path::new(&plugin_dir).exists() {
        std::process::Command::new("git")
            .args(["clone", source, &plugin_dir])
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
