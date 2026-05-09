pub mod handlers;
mod parsers;

use crate::config::Config;
use clap::{Parser, Subcommand};
use std::fmt::Debug;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(
    about = env!("CARGO_PKG_DESCRIPTION"),
    version = env!("CARGO_PKG_VERSION"),
    long_version = concat!(
        env!("CARGO_PKG_VERSION"),
        "+",
        env!("VERGEN_GIT_SHA"),
        " (built on ",
        env!("VERGEN_BUILD_DATE"),
        ")\n\n",
        "Repository: ",
        env!("CARGO_PKG_REPOSITORY"),
        "\nIssue tracker: ",
        env!("CARGO_PKG_REPOSITORY"),
        "/issues"
    )
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
        #[arg(short = 'a', long, value_parser = parsers::validate_keystore_alias)]
        keystore_alias: Option<String>,

        /// The password for the keystore.
        #[arg(short = 'p', long, value_parser = parsers::validate_keystore_password)]
        keystore_password: Option<String>,

        /// The number of parallel jobs to use for compilation, between 1 and 8.
        #[arg(short = 'j', long, value_parser = parsers::validate_apktool_jobs)]
        jobs: Option<usize>,
    },

    /// Decompile an APK file into a smali directory.
    #[command(visible_alias = "d")]
    Decompile {
        /// The input APK file.
        input: PathBuf,

        /// The output directory for the decompiled smali files. If not specified, a directory with the same name as the APK will be created in the current directory.
        out_dir: Option<PathBuf>,

        /// The number of parallel jobs to use for decompilation, between 1 and 8.
        #[arg(short = 'j', long, value_parser = parsers::validate_apktool_jobs)]
        jobs: Option<usize>,
    },

    /// Generate a keystore file for signing APKs.
    #[command(visible_alias = "k")]
    Keygen {
        /// The keystore alias to use.
        #[arg(short = 'a', long, value_parser = parsers::validate_keystore_alias)]
        keystore_alias: Option<String>,

        /// The password for the keystore.
        #[arg(short = 'p', long, value_parser = parsers::validate_keystore_password)]
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
        #[arg(value_parser = parsers::trim_string)]
        value: String,
    },

    /// Delete a configuration key.
    #[command(visible_aliases = ["del", "rm", "unset", "remove"])]
    Delete {
        /// The configuration key to delete.
        key: Config,
    },

    /// Show where the configuration file is located.
    #[command(visible_aliases = ["loc", "path", "where", "position", "pos"])]
    Location,
}
