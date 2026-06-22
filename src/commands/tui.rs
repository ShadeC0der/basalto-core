pub fn run() {
    let status = std::process::Command::new("basalto-tui").status();
    if status.is_err() {
        println!("basalto-tui no esta instalado. Corre: basalto update");
    }
}
