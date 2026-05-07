mod cli;
mod config;
mod constants;
mod utils;

use clap::Parser;
use cli::{Cli, handlers};

fn main() -> anyhow::Result<()> {
    let args = Cli::parse();

    if !matches!(args.command, cli::Commands::Config { .. }) {
        config::Config::validate_all()?;
    }

    match args.command {
        cli::Commands::Compile {
            input_dir,
            out_file,
            keystore_alias,
            keystore_password,
            jobs,
        } => {
            handlers::compile(input_dir, out_file, keystore_alias, keystore_password, jobs)?;
            Ok(())
        }

        cli::Commands::Decompile {
            input,
            out_dir,
            jobs,
        } => {
            handlers::decompile(input, out_dir, jobs)?;
            Ok(())
        }

        cli::Commands::Keygen {
            keystore_alias,
            keystore_password,
        } => {
            handlers::keygen(keystore_alias, keystore_password)?;
            Ok(())
        }

        cli::Commands::Config { action } => match action {
            cli::ConfigAction::Get { key } => match key.get()? {
                Some(value) => {
                    println!("{}", value);
                    Ok(())
                }
                None => anyhow::bail!("{} not configured.", key),
            },

            cli::ConfigAction::Set { key, value } => {
                key.set(&value)?;
                Ok(())
            }

            cli::ConfigAction::Delete { key } => {
                key.delete()?;
                Ok(())
            }

            cli::ConfigAction::Location => {
                println!("{}", config::config_file_path().display());
                Ok(())
            }
        },
    }
}
