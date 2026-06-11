mod config;
mod plugins;

fn main() {
    let config = config::read_config();
    println!("{}", config.library.url);

    let plugin = plugins::read_plugins();

    println!("{}", plugin[0].source);
}
