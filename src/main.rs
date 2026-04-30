mod config;

fn main() {
    match config::Config::JavaPath.get() {
        Some(value) => println!("Java Path: {value}"),
        None => println!("Java Path not found in config file."),
    }

    match config::Config::JavaPath.set("C:\\Program Files\\Java\\jdk-17.0.1\\bin\\java.exe") {
        Ok(_) => println!("Java Path updated successfully."),
        Err(e) => println!("Failed to update Java Path: {e}"),
    }

    match config::Config::JavaPath.get() {
        Some(value) => println!("Java Path: {value}"),
        None => println!("Java Path not found in config file."),
    }

    match config::Config::JavaPath.delete() {
        Ok(_) => println!("Java Path deleted successfully."),
        Err(e) => println!("Failed to delete Java Path: {e}"),
    }

    match config::Config::JavaPath.get() {
        Some(value) => println!("Java Path: {value}"),
        None => println!("Java Path not found in config file."),
    }
}
