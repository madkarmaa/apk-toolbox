pub mod executors;

use crate::config::Config;
use clap::{Parser, Subcommand};
use std::fmt::Debug;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(
    name = env!("CARGO_PKG_NAME"),
    bin_name = env!("CARGO_PKG_NAME"),
    version = env!("CARGO_PKG_VERSION"),
    about = env!("CARGO_PKG_DESCRIPTION")
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Compile a smali directory into a ready-to-use APK file.
    Compile {
        /// The input smali directory.
        input_dir: PathBuf,
        /// The output directory for the compiled APK. If not specified, the APK will be created in the current directory.
        out_dir: Option<PathBuf>,

        /// The keystore alias to use.
        #[arg(short = 'a', long)]
        keystore_alias: Option<String>,

        /// The password for the keystore.
        #[arg(short = 'p', long)]
        keystore_password: Option<String>,
    },

    /// Decompile an APK file into a smali directory.
    Decompile {
        /// The input APK file.
        input: PathBuf,
        /// The output directory for the decompiled smali files. If not specified, a directory with the same name as the APK will be created in the current directory.
        out_dir: Option<PathBuf>,
    },

    /// Generate a keystore file for signing APKs.
    Keygen {
        /// The keystore alias to use.
        #[arg(short = 'a', long)]
        keystore_alias: Option<String>,

        /// The password for the keystore.
        #[arg(short = 'p', long)]
        keystore_password: Option<String>,
    },

    /// Manage configuration settings.
    Config {
        #[command(subcommand)]
        action: ConfigAction,
    },
}

#[derive(Subcommand, Debug)]
pub enum ConfigAction {
    /// Get the value of a configuration key.
    Get {
        /// The configuration key to retrieve.
        key: Config,
    },

    /// Set the value of a configuration key.
    Set {
        /// The configuration key to set.
        key: Config,

        /// The value to set for the configuration key.
        value: String,
    },

    /// Delete a configuration key.
    Delete {
        /// The configuration key to delete.
        key: Config,
    },
}
