use crate::utils::root_dir;
use clap::ValueEnum;
use std::collections::HashMap;
use std::env;
use std::fmt::Debug;
use std::fs;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use std::sync::{OnceLock, RwLock};
use strum_macros::{AsRefStr, Display, EnumString};

pub const CONFIG_FILE_NAME: &str = concat!(".", env!("CARGO_PKG_NAME"));
static CONFIG_CACHE: OnceLock<RwLock<HashMap<String, String>>> = OnceLock::new();

fn config_file_path() -> PathBuf {
    let mut root = root_dir();
    root.push(CONFIG_FILE_NAME);
    root
}

#[derive(AsRefStr, Display, EnumString, Debug, Clone, ValueEnum)]
pub enum Config {
    #[strum(serialize = "java.path")]
    #[clap(name = "java.path")]
    JavaPath,

    #[strum(serialize = "apktool.path")]
    #[clap(name = "apktool.path")]
    ApktoolPath,

    #[strum(serialize = "apkeditor.path")]
    #[clap(name = "apkeditor.path")]
    ApkeditorPath,

    #[strum(serialize = "apksigner.path")]
    #[clap(name = "apksigner.path")]
    ApksignerPath,

    #[strum(serialize = "zipalign.path")]
    #[clap(name = "zipalign.path")]
    ZipalignPath,

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

        Self::save_to_disk()
    }

    pub fn delete(&self) -> io::Result<()> {
        Self::cache().write().unwrap().remove(self.as_ref());
        Self::save_to_disk()
    }

    fn save_to_disk() -> io::Result<()> {
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
