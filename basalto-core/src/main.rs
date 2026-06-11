mod config;

fn main() {
    let config = config::read_config();
    println!("{}", config.library.url);
}
