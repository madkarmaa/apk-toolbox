use crate::utils::root_dir;
use std::collections::HashMap;
use std::env;
use std::fmt;
use std::fmt::Debug;
use std::fs;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::{OnceLock, RwLock};

pub const CONFIG_FILE_NAME: &str = concat!(".", env!("CARGO_PKG_NAME"));

fn config_file_path() -> PathBuf {
    let mut root = root_dir();
    root.push(CONFIG_FILE_NAME);
    root
}

static CONFIG_CACHE: OnceLock<RwLock<HashMap<String, String>>> = OnceLock::new();

#[derive(Debug, Clone)]
pub enum Config {
    JavaPath,
    ApktoolPath,
    ApkeditorPath,
    ApksignerPath,
    ZipalignPath,
    KeystorePath,
    KeystoreAlias,
    KeystorePassword,
}

impl FromStr for Config {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "java.path" => Ok(Config::JavaPath),
            "apktool.path" => Ok(Config::ApktoolPath),
            "apkeditor.path" => Ok(Config::ApkeditorPath),
            "apksigner.path" => Ok(Config::ApksignerPath),
            "zipalign.path" => Ok(Config::ZipalignPath),
            "keystore.path" => Ok(Config::KeystorePath),
            "keystore.alias" => Ok(Config::KeystoreAlias),
            "keystore.password" => Ok(Config::KeystorePassword),

            _ => Err(format!("Invalid configuration key '{}'.", value)),
        }
    }
}

impl AsRef<str> for Config {
    fn as_ref(&self) -> &str {
        match self {
            Config::JavaPath => "java.path",
            Config::ApktoolPath => "apktool.path",
            Config::ApkeditorPath => "apkeditor.path",
            Config::ApksignerPath => "apksigner.path",
            Config::ZipalignPath => "zipalign.path",
            Config::KeystorePath => "keystore.path",
            Config::KeystoreAlias => "keystore.alias",
            Config::KeystorePassword => "keystore.password",
        }
    }
}

impl fmt::Display for Config {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "{}", self.as_ref())
    }
}

impl Config {
    fn read_config_file() -> io::Result<HashMap<String, String>> {
        let file = File::open(config_file_path())?;
        let reader = BufReader::new(file);
        let mut config = HashMap::new();

        for line_res in reader.lines() {
            let line = line_res?;
            if let Some((key, value)) = line.split_once('=') {
                config.insert(key.trim().to_string(), value.trim().to_string());
            }
        }

        Ok(config)
    }

    fn cache() -> &'static RwLock<HashMap<String, String>> {
        CONFIG_CACHE.get_or_init(|| {
            let map = Self::read_config_file()
                .unwrap_or_default()
                .into_iter()
                .collect();

            RwLock::new(map)
        })
    }

    pub fn get(&self) -> Option<String> {
        Self::cache().read().unwrap().get(self.as_ref()).cloned()
    }

    pub fn set(&self, value: &str) -> io::Result<()> {
        Self::cache()
            .write()
            .unwrap()
            .insert(self.to_string(), value.to_string());

        Self::flush()
    }

    pub fn delete(&self) -> io::Result<()> {
        Self::cache().write().unwrap().remove(self.as_ref());
        Self::flush()
    }

    fn flush() -> io::Result<()> {
        let config_path = config_file_path();
        let tmp = config_path.with_extension("tmp");

        let contents = Self::cache()
            .read()
            .unwrap()
            .iter()
            .map(|(config_key, value)| format!("{}={}", config_key, value))
            .collect::<Vec<String>>()
            .join("\n");

        fs::write(&tmp, contents)?;
        fs::rename(&tmp, config_path)?;
        Ok(())
    }
}
