pub mod handlers;

use crate::config::Config;
use clap::{Parser, Subcommand};
use std::fmt::Debug;
use std::path::PathBuf;

fn trim_string(v: &str) -> Result<String, String> {
    Ok(v.trim().to_string())
}

fn validate_keystore_alias(v: &str) -> Result<String, String> {
    let trimmed = v.trim();
    if trimmed.is_empty() {
        Err("Keystore alias cannot be empty".to_string())
    } else {
        Ok(trimmed.to_string())
    }
}

fn validate_keystore_password(v: &str) -> Result<String, String> {
    let trimmed = v.trim();
    if trimmed.len() < 6 {
        Err("Keystore password must be at least 6 characters".to_string())
    } else {
        Ok(trimmed.to_string())
    }
}

#[derive(Parser, Debug)]
#[command(
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
    #[command(visible_alias = "c")]
    Compile {
        /// The input smali directory.
        input_dir: PathBuf,

        /// The output file for the compiled APK. If not specified, the APK will be created in the current directory.
        out_file: Option<PathBuf>,

        /// The keystore alias to use.
        #[arg(short = 'a', long, value_parser = validate_keystore_alias)]
        keystore_alias: Option<String>,

        /// The password for the keystore.
        #[arg(short = 'p', long, value_parser = validate_keystore_password)]
        keystore_password: Option<String>,

        /// The number of parallel jobs to use for compilation. If not specified, the number of CPU cores will be used.
        #[arg(short = 'j', long)]
        jobs: Option<usize>,

        /// The JVM heap size to use for compilation.
        #[arg(long, default_value = "2G", value_parser = trim_string)]
        jvm_heap: String,
    },

    /// Decompile an APK file into a smali directory.
    #[command(visible_alias = "d")]
    Decompile {
        /// The input APK file.
        input: PathBuf,

        /// The output directory for the decompiled smali files. If not specified, a directory with the same name as the APK will be created in the current directory.
        out_dir: Option<PathBuf>,

        /// The number of parallel jobs to use for decompilation. If not specified, the number of CPU cores will be used.
        #[arg(short = 'j', long)]
        jobs: Option<usize>,

        /// The JVM heap size to use for decompilation.
        #[arg(long, default_value = "2G", value_parser = trim_string)]
        jvm_heap: String,
    },

    /// Generate a keystore file for signing APKs.
    #[command(visible_alias = "k")]
    Keygen {
        /// The keystore alias to use.
        #[arg(short = 'a', long, value_parser = validate_keystore_alias)]
        keystore_alias: Option<String>,

        /// The password for the keystore.
        #[arg(short = 'p', long, value_parser = validate_keystore_password)]
        keystore_password: Option<String>,
    },

    /// Manage configuration settings.
    #[command(visible_alias = "cfg")]
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
    #[command(visible_aliases = ["put", "add", "update"])]
    Set {
        /// The configuration key to set.
        key: Config,

        /// The value to set for the configuration key.
        #[arg(value_parser = trim_string)]
        value: String,
    },

    /// Delete a configuration key.
    #[command(visible_aliases = ["del", "rm", "unset", "remove"])]
    Delete {
        /// The configuration key to delete.
        key: Config,
    },

    /// Show where the configuration file is located.
    #[command(visible_alias = "loc")]
    Location,
}
