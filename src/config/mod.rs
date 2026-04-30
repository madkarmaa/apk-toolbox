use std::fmt;
use std::fs::{File, write};
use std::io::{BufRead, BufReader, Result};

pub const CONFIG_FILE_NAME: &str = concat!(".", env!("CARGO_PKG_NAME"));

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

impl fmt::Display for Config {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        let key_value = match self {
            Config::JavaPath => "java.path",
            Config::ApktoolPath => "apktool.path",
            Config::ApkeditorPath => "apkeditor.path",
            Config::ApksignerPath => "apksigner.path",
            Config::ZipalignPath => "zipalign.path",
            Config::KeystorePath => "keystore.path",
            Config::KeystoreAlias => "keystore.alias",
            Config::KeystorePassword => "keystore.password",
        };
        write!(formatter, "{key_value}")
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

impl Config {
    fn read_config_file() -> Result<Vec<(String, String)>> {
        let file = File::open(CONFIG_FILE_NAME)?;
        let reader = BufReader::new(file);
        let mut config = Vec::new();

        for line_res in reader.lines() {
            let line = line_res?;
            if let Some((key, value)) = line.split_once('=') {
                config.push((key.trim().to_string(), value.trim().to_string()));
            }
        }

        Ok(config)
    }

    pub fn get(&self) -> Option<String> {
        let config = Self::read_config_file().unwrap_or_else(|_| Vec::new());
        let key_str = self.as_ref();

        config
            .into_iter()
            .find_map(|(k, v)| (k == key_str).then_some(v))
    }

    pub fn set(&self, value: &str) -> Result<()> {
        let mut config = Self::read_config_file().unwrap_or_else(|_| Vec::new());
        let key_str = self.as_ref();

        if let Some((_, v)) = config.iter_mut().find(|(k, _)| k == key_str) {
            *v = value.to_string();
        } else {
            config.push((key_str.to_string(), value.to_string()));
        }

        write(
            CONFIG_FILE_NAME,
            config
                .into_iter()
                .map(|(k, v)| format!("{}={}", k, v))
                .collect::<Vec<String>>()
                .join("\n"),
        )
    }

    pub fn delete(&self) -> Result<()> {
        let mut config = Self::read_config_file().unwrap_or_else(|_| Vec::new());
        let key_str = self.as_ref();

        config.retain(|(k, _)| k != key_str);

        write(
            CONFIG_FILE_NAME,
            config
                .into_iter()
                .map(|(k, v)| format!("{}={}", k, v))
                .collect::<Vec<String>>()
                .join("\n"),
        )
    }
}
