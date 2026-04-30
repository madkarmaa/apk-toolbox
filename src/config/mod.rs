use std::fmt;

pub const CONFIG_FILE_NAME: &str = concat!(".", env!("CARGO_PKG_NAME"));

pub enum ConfigKey {
    JavaPath,
    ApktoolPath,
    ApkeditorPath,
    ApksignerPath,
    ZipalignPath,
    KeystorePath,
    KeystoreAlias,
    KeystorePassword,
}

impl fmt::Display for ConfigKey {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        let key_value = match self {
            ConfigKey::JavaPath => "java.path",
            ConfigKey::ApktoolPath => "apktool.path",
            ConfigKey::ApkeditorPath => "apkeditor.path",
            ConfigKey::ApksignerPath => "apksigner.path",
            ConfigKey::ZipalignPath => "zipalign.path",
            ConfigKey::KeystorePath => "keystore.path",
            ConfigKey::KeystoreAlias => "keystore.alias",
            ConfigKey::KeystorePassword => "keystore.password",
        };
        write!(formatter, "{key_value}")
    }
}

impl AsRef<str> for ConfigKey {
    fn as_ref(&self) -> &str {
        match self {
            ConfigKey::JavaPath => "java.path",
            ConfigKey::ApktoolPath => "apktool.path",
            ConfigKey::ApkeditorPath => "apkeditor.path",
            ConfigKey::ApksignerPath => "apksigner.path",
            ConfigKey::ZipalignPath => "zipalign.path",
            ConfigKey::KeystorePath => "keystore.path",
            ConfigKey::KeystoreAlias => "keystore.alias",
            ConfigKey::KeystorePassword => "keystore.password",
        }
    }
}
