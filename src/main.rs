//mod config; // Lee configuración en ./basalto/config.toml
mod dispatcher; // Carga cada .so y crea el mapa comando -> plugin
mod installer; // Clona repo del plugin y compila el .so, declarado aquí para usarlo en dispatcher
mod plugins; // Lee lista de plugins en ./basalto/plugin/*.toml
mod setup; // Crea carpetas iniciales en ./basalto

fn main() {
    /* Resumen de main()
     * Captura los args y detecta si viene el flag --time
     * Crea Carpetas necesarias en .basalto en caso que no existan
     * Crea un vector dinámico con los plugin declarados en ./basalto/plugins/'*toml'
     * Crea un mapa de comandos de los plugins y guarda los .so en (_lib) para que map funcione
     * En el if si no hay comando, muestra como se utiliza
     * Si hay comando, lo busca en el mapa y lo despacha al plugin correspondiente
     * Si --time estaba presente, imprime el tiempo de cada fase
     */

    let args: Vec<String> = std::env::args().collect();
    let time_flag = args.contains(&"--time".to_string());
    let args: Vec<String> = args.into_iter().filter(|a| a != "--time").collect();

    let t0 = std::time::Instant::now();
    setup::run();
    let t1 = std::time::Instant::now();

    // let _config = config::read_config();
    let plugins = plugins::read_plugins();
    let t2 = std::time::Instant::now();

    let (map, _libs) = dispatcher::build(&plugins);
    let t3 = std::time::Instant::now();

    let t4;
    let t5;

    if args.len() < 2 {
        t4 = std::time::Instant::now();
        println!("Usage: basalto <command>");
        t5 = std::time::Instant::now();
    } else {
        let command = &args[1];
        let arguments = &args[2..];
        let args_str: Vec<&str> = arguments.iter().map(|a| a.as_str()).collect();

        t4 = std::time::Instant::now();
        match map.get(command.as_str()) {
            Some(plugin) => plugin.execute_command(command, &args_str),
            None => println!("Unknown command: {}", command),
        }
        t5 = std::time::Instant::now();
    }

    if time_flag {
        println!("  setup      → {:>8.3}ms", (t1 - t0).as_secs_f64() * 1000.0);
        println!("  plugins    → {:>8.3}ms", (t2 - t1).as_secs_f64() * 1000.0);
        println!("  dispatcher → {:>8.3}ms", (t3 - t2).as_secs_f64() * 1000.0);
        println!("  execute    → {:>8.3}ms", (t5 - t4).as_secs_f64() * 1000.0);
        println!("  {}", "─".repeat(28));
        println!("  total      → {:>8.3}ms", (t5 - t0).as_secs_f64() * 1000.0);
    }

    drop(map);
    drop(_libs);
}
