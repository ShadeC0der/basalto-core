//mod config; // Lee configuración en ./basalto/config.toml
mod dispatcher; // Carga cada .so y crea el mapa comando -> plugin
mod installer; // Clona repo del plugin y compila el .so, declarado aquí para usarlo en dispatcher
mod plugins; // Lee lista de plugins en ./basalto/plugin/*.toml
mod setup; // Crea carpetas iniciales en ./basalto

fn main() {
    /* Resumen de main()
     * Crea Carpetas necesarias en .basalto en caso que no existan
     * Crea un vector dinámico con los plugin declarados en ./basalto/plugins/'*toml'
     * Crea un mapa de comandos de los plugins y guarda los .so en (_lib) para que map funcione
     * Captura los comandos que ingresa el usuario con esta estructura [basalto, command, args_str]
     * En el if si no hay comando, muestra como se utiliza
     * Si hay comando, lo busca en el mapa y lo despacha al plugin correspondiente
     */

    setup::run();

    // let _config = config::read_config();
    let plugins = plugins::read_plugins();
    let (map, _libs) = dispatcher::build(&plugins);

    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("Usage: basalto <command>");
    } else {
        let command = &args[1];
        let arguments = &args[2..];
        let args_str: Vec<&str> = arguments.iter().map(|a| a.as_str()).collect();

        match map.get(command.as_str()) {
            Some(plugin) => plugin.execute_command(command, &args_str),
            None => println!("Unknown command: {}", command),
        }
    }
}
