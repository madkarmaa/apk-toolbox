use std::collections::HashMap;
use std::env;
use std::fmt;
use std::fs;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use std::sync::{OnceLock, RwLock};

pub const CONFIG_FILE_NAME: &str = concat!(".", env!("CARGO_PKG_NAME"));

fn get_config_file_path() -> PathBuf {
    let mut path: PathBuf;

    if cfg!(debug_assertions) {
        path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    } else {
        path = env::current_exe().unwrap_or_else(|_| PathBuf::from("."));
        path.pop();
    }

    path.push(CONFIG_FILE_NAME);
    path
}

static CONFIG_CACHE: OnceLock<RwLock<HashMap<String, String>>> = OnceLock::new();

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
        let file = File::open(get_config_file_path())?;
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
        let config_path = get_config_file_path();
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
