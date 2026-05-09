pub mod validators;

use crate::constants::errors::AppError;
use crate::utils::{format_validation_error, root_dir};
use clap::ValueEnum;
use serde::{Deserialize, Serialize};
use serde_valid::Validate;
use std::env;
use std::fmt::Debug;
use std::fs;
use std::io;
use std::path::PathBuf;
use std::sync::{OnceLock, RwLock};
use strum_macros::{AsRefStr, Display, EnumString};
use validators::*;

pub const CONFIG_FILE_NAME: &str = concat!(env!("CARGO_PKG_NAME"), ".config.toml");
static CONFIG_CACHE: OnceLock<RwLock<AppConfig>> = OnceLock::new();

pub fn config_file_path() -> PathBuf {
    root_dir().join(CONFIG_FILE_NAME)
}

#[derive(Debug, Default, Serialize, Deserialize, Validate, Clone)]
struct AppConfig {
    #[serde(default)]
    #[validate]
    pub java: JavaConfig,

    #[serde(default)]
    #[validate]
    pub apktool: ApktoolConfig,

    #[serde(default)]
    #[validate]
    pub apkeditor: ApkeditorConfig,

    #[serde(default)]
    #[validate]
    pub build_tools: BuildToolsConfig,

    #[serde(default)]
    #[validate]
    pub keystore: KeystoreConfig,
}

#[derive(Debug, Default, Serialize, Deserialize, Validate, Clone)]
pub struct JavaConfig {
    #[validate(custom = validate_java_home)]
    pub home: Option<String>,
}

#[derive(Debug, Default, Serialize, Deserialize, Validate, Clone)]
pub struct ApktoolConfig {
    #[validate(custom = validate_jar_path)]
    pub path: Option<String>,
}

#[derive(Debug, Default, Serialize, Deserialize, Validate, Clone)]
pub struct ApkeditorConfig {
    #[validate(custom = validate_jar_path)]
    pub path: Option<String>,
}

#[derive(Debug, Default, Serialize, Deserialize, Validate, Clone)]
pub struct BuildToolsConfig {
    #[validate(custom = validate_build_tools_path)]
    pub path: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Validate, Clone)]
#[serde(default)]
pub struct KeystoreConfig {
    #[validate(custom = validate_keystore_path)]
    pub path: Option<String>,

    pub alias: Option<String>,

    #[validate(
        min_length = 6,
        message = "Keystore password must be at least 6 characters long."
    )]
    pub password: Option<String>,
}

impl Default for KeystoreConfig {
    fn default() -> Self {
        Self {
            path: Some(
                crate::utils::root_dir()
                    .join("keystore.jks")
                    .to_string_lossy()
                    .to_string(),
            ),

            alias: None,
            password: None,
        }
    }
}

#[derive(AsRefStr, Display, EnumString, Debug, Clone, ValueEnum)]
pub enum Config {
    #[strum(serialize = "java.home")]
    #[clap(name = "java.home")]
    JavaHome,

    #[strum(serialize = "apktool.path")]
    #[clap(name = "apktool.path")]
    ApktoolPath,

    #[strum(serialize = "apkeditor.path")]
    #[clap(name = "apkeditor.path")]
    ApkeditorPath,

    #[strum(serialize = "build_tools.path")]
    #[clap(name = "build_tools.path")]
    BuildToolsPath,

    #[strum(serialize = "keystore.path")]
    #[clap(name = "keystore.path")]
    KeystorePath,

    #[strum(serialize = "keystore.alias")]
    #[clap(name = "keystore.alias")]
    KeystoreAlias,

    #[strum(serialize = "keystore.password")]
    #[clap(name = "keystore.password")]
    KeystorePassword,
}

impl Config {
    fn read_config_file() -> Result<AppConfig, AppError> {
        let path = config_file_path();
        if !path.exists() {
            return Ok(AppConfig::default());
        }

        let content = fs::read_to_string(path)?;

        let config: AppConfig =
            toml::from_str(&content).map_err(|e| AppError::Config(e.to_string()))?;

        Ok(config)
    }

    fn cache() -> Result<&'static RwLock<AppConfig>, AppError> {
        if let Some(cache) = CONFIG_CACHE.get() {
            return Ok(cache);
        }
        let config = Self::read_config_file()?;
        Ok(CONFIG_CACHE.get_or_init(|| RwLock::new(config)))
    }

    pub fn validate_all() -> Result<(), AppError> {
        let cache = Self::cache()?
            .read()
            .expect("Failed to read from config cache");

        cache
            .validate()
            .map_err(|e| AppError::Config(format_validation_error(&e)))
    }

    pub fn get(&self) -> Result<String, AppError> {
        let cache = Self::cache()?
            .read()
            .expect("Failed to read from config cache");

        match self {
            Config::JavaHome => cache
                .java
                .home
                .clone()
                .ok_or(AppError::JavaHomeNotConfigured),

            Config::ApktoolPath => cache
                .apktool
                .path
                .clone()
                .ok_or(AppError::ApktoolPathNotConfigured),

            Config::ApkeditorPath => cache
                .apkeditor
                .path
                .clone()
                .ok_or(AppError::ApkeditorPathNotConfigured),

            Config::BuildToolsPath => cache
                .build_tools
                .path
                .clone()
                .ok_or(AppError::BuildToolsPathNotConfigured),

            Config::KeystorePath => cache
                .keystore
                .path
                .clone()
                .ok_or(AppError::KeystorePathExpected),

            Config::KeystoreAlias => cache
                .keystore
                .alias
                .clone()
                .ok_or(AppError::KeystoreAliasNotFound),

            Config::KeystorePassword => cache
                .keystore
                .password
                .clone()
                .ok_or(AppError::KeystorePasswordNotFound),
        }
    }

    pub fn set(&self, value: &str) -> Result<(), AppError> {
        let mut cache = Self::cache()?
            .write()
            .expect("Failed to write to config cache");

        let mut new_config = cache.clone();

        match self {
            Config::JavaHome => new_config.java.home = Some(value.to_string()),
            Config::ApktoolPath => new_config.apktool.path = Some(value.to_string()),
            Config::ApkeditorPath => new_config.apkeditor.path = Some(value.to_string()),
            Config::BuildToolsPath => new_config.build_tools.path = Some(value.to_string()),
            Config::KeystorePath => new_config.keystore.path = Some(value.to_string()),
            Config::KeystoreAlias => new_config.keystore.alias = Some(value.to_string()),
            Config::KeystorePassword => new_config.keystore.password = Some(value.to_string()),
        }

        new_config
            .validate()
            .map_err(|e| AppError::Config(format_validation_error(&e)))?;

        *cache = new_config;
        Self::save_to_disk(&cache).map_err(|e| AppError::Config(e.to_string()))
    }

    pub fn delete(&self) -> Result<(), AppError> {
        let mut cache = Self::cache()?
            .write()
            .expect("Failed to write to config cache");

        match self {
            Config::JavaHome => cache.java.home = None,
            Config::ApktoolPath => cache.apktool.path = None,
            Config::ApkeditorPath => cache.apkeditor.path = None,
            Config::BuildToolsPath => cache.build_tools.path = None,
            Config::KeystorePath => cache.keystore.path = None,
            Config::KeystoreAlias => cache.keystore.alias = None,
            Config::KeystorePassword => cache.keystore.password = None,
        }

        Self::save_to_disk(&cache).map_err(|e| AppError::Config(e.to_string()))
    }

    fn save_to_disk(config: &AppConfig) -> io::Result<()> {
        let config_path = config_file_path();
        let tmp = config_path.with_extension("tmp");

        let contents = toml::to_string_pretty(config)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

        fs::write(&tmp, contents)?;
        fs::rename(&tmp, config_path)?;
        Ok(())
    }
}
