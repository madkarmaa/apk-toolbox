use crate::config::Config;
use crate::constants::errors;
use crate::utils;
use std::path::PathBuf;

pub fn keygen(
    keystore_alias: Option<String>,
    keystore_password: Option<String>,
) -> Result<(), String> {
    let keystore_path = Config::KeystorePath
        .get()
        .expect("Keystore path should have a default value");

    let keystore_alias = keystore_alias
        .or_else(|| Config::KeystoreAlias.get())
        .ok_or_else(|| errors::KEYSTORE_ALIAS_NOT_FOUND.to_string())?;

    let keystore_password = keystore_password
        .or_else(|| Config::KeystorePassword.get())
        .ok_or_else(|| errors::KEYSTORE_PASSWORD_NOT_FOUND.to_string())?;

    let java_home = Config::JavaHome
        .get()
        .ok_or_else(|| errors::JAVA_HOME_NOT_CONFIGURED.to_string())?;

    let executable_name = if cfg!(target_os = "windows") {
        "keytool.exe"
    } else {
        "keytool"
    };

    let keytool_path = PathBuf::from(java_home).join("bin").join(executable_name);

    println!("Generating keystore...");

    utils::execute_blocking(
        &keytool_path.to_string_lossy(),
        &[
            "-genkey",
            "-keystore",
            &keystore_path,
            "-keyalg",
            "RSA",
            "-keysize",
            "4096",
            "-validity",
            "99999",
            "-alias",
            &keystore_alias,
            "-storepass",
            &keystore_password,
            "-keypass",
            &keystore_password,
            "-dname",
            "CN=Unknown, OU=Unknown, O=Unknown, L=Unknown, S=Unknown, C=Unknown",
        ],
    )
    .map_err(|err| err.to_string())?;

    println!("Keystore generated successfully at {}", keystore_path);
    Ok(())
}
