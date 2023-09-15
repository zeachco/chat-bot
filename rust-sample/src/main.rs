use config::Config;

mod config;

fn main() {
    let config = Config::from_env();
    println!("{:?}", config);
}
