mod cli;
mod config;
mod constants;
mod utils;

use clap::Parser;
use cli::{Cli, executors};

fn main() -> anyhow::Result<()> {
    let args = Cli::parse();

    match args.command {
        cli::Commands::Compile {
            input_dir,
            out_file,
            keystore_alias,
            keystore_password,
            jobs,
        } => {
            executors::compile(input_dir, out_file, keystore_alias, keystore_password, jobs)?;
            Ok(())
        }

        cli::Commands::Decompile {
            input,
            out_dir,
            jobs,
        } => {
            executors::decompile(input, out_dir, jobs)?;
            Ok(())
        }

        cli::Commands::Keygen {
            keystore_alias,
            keystore_password,
        } => {
            executors::keygen(keystore_alias, keystore_password)?;
            Ok(())
        }

        cli::Commands::Config { action } => match action {
            cli::ConfigAction::Get { key } => match key.get() {
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
        },
    }
}
