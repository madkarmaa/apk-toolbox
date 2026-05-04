mod cli;
mod config;
mod constants;
mod utils;

use clap::Parser;
use cli::{Cli, executors};
use std::process;

fn main() {
    let args = Cli::parse();

    match args.command {
        cli::Commands::Compile {
            input,
            out_dir,
            keystore_alias,
            keystore_password,
        } => {}

        cli::Commands::Decompile { input, out_dir } => {}

        cli::Commands::Keygen {
            keystore_alias,
            keystore_password,
        } => {
            if let Err(err) = executors::keygen(keystore_alias, keystore_password) {
                eprintln!("{}", err);
                process::exit(1);
            }
        }

        cli::Commands::Config { action } => match action {
            cli::ConfigAction::Get { key } => match key.get() {
                Some(value) => println!("{}", value),
                None => {
                    eprintln!("{} not configured.", key);
                    process::exit(1);
                }
            },

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
