mod config;
mod dispatcher;
mod installer;
mod plugins;
mod setup;

fn main() {
    setup::run();
    let _config = config::read_config();
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
