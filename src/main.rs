mod cli;
mod config;
mod utils;

use clap::Parser;
use cli::{Cli, executors};
use std::process;

fn main() {
    let cli = Cli::parse();

    match cli.command {
        cli::Commands::Compile { input, out_dir } => {
            println!("Compiling {:?} to {:?}", input, out_dir);
        }

        cli::Commands::Decompile { input, out_dir } => {
            println!("Decompiling {:?} to {:?}", input, out_dir);
        }

        cli::Commands::Keygen => {
            if let Err(err) = executors::keygen() {
                eprintln!("{}", err);
                process::exit(1);
            }
        }

        cli::Commands::Config { action } => match action {
            cli::ConfigAction::Get { key } => {
                let value = key.get().expect("Config key should be defined");
                println!("{}", value);
            }

            cli::ConfigAction::Set { key, value } => {
                if let Err(err) = key.set(&value) {
                    eprintln!("{}", err);
                    process::exit(1);
                }
            }

            cli::ConfigAction::Delete { key } => {
                if let Err(err) = key.delete() {
                    eprintln!("{}", err);
                    process::exit(1);
                }
            }
        },
    }
}
