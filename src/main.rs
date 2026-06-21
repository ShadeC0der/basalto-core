//mod config; // Lee configuración en ./basalto/config.toml
mod commands; // Comandos built-in del Core
mod dispatcher; // Carga cada .so y crea el mapa comando -> plugin
mod installer; // Clona repo del plugin y compila el .so, declarado aquí para usarlo en dispatcher
mod plugins; // Lee lista de plugins en ./basalto/plugin/*.toml
mod setup; // Crea carpetas iniciales en ./basalto

fn main() {
    /* Resumen de main()
     * Captura los argumentos de la terminal
     * Si el usuario pasó --version, imprime la versión del core y termina
     * Inicializa las carpetas con setup::run()
     * Lee los plugins declarados en ~/.basalto/plugins/
     * Construye el mapa de comandos con dispatcher::build()
     * Si no hay argumentos, muestra el uso
     * Si hay argumentos, despacha al comando built-in o al plugin correspondiente
     */

    let args: Vec<String> = std::env::args().collect();

    if args.iter().any(|a| a == "--version") {
        println!("basalto-core v{}", env!("CARGO_PKG_VERSION"));
        return;
    }

    setup::run();

    // let _config = config::read_config();
    let plugins = plugins::read_plugins();
    let (map, _libs) = dispatcher::build(&plugins);

    if args.len() < 2 {
        println!("Usage: basalto <command>");
    } else {
        let command = &args[1];
        let arguments = &args[2..];
        let args_str: Vec<&str> = arguments.iter().map(|a| a.as_str()).collect();

        match command.as_str() {
            "version" => commands::version::run(&plugins, &args_str),
            _ => match map.get(command.as_str()) {
                Some(plugin) => plugin.execute_command(command, &args_str),
                None => println!("Unknown command: {}", command),
            },
        }
    }

    drop(map);
    drop(_libs);
}
