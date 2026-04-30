mod config;

fn main() {
    println!("{}", config::ConfigKey::JavaPath.as_ref());
}
