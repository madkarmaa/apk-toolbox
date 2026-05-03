use crate::config::Config;
use crate::utils;
use std::path::PathBuf;

pub fn keygen() -> Result<(), String> {
    let keystore_path = Config::KeystorePath.get().ok_or_else(|| {
        "Keystore path not configured. Please configure it using the config command.".to_string()
    })?;

    let keystore_alias = Config::KeystoreAlias.get().ok_or_else(|| {
        "Keystore alias not configured. Please configure it using the config command.".to_string()
    })?;

    let keystore_password = Config::KeystorePassword.get().ok_or_else(|| {
        "Keystore password not configured. Please configure it using the config command."
            .to_string()
    })?;

    let javabase_path = Config::JavaPath.get().ok_or_else(|| {
        "Java path not configured. Please configure it using the config command.".to_string()
    })?;

    let executable_name = if cfg!(target_os = "windows") {
        "keytool.exe"
    } else {
        "keytool"
    };

    let keytool_path = PathBuf::from(javabase_path)
        .join("bin")
        .join(executable_name);

    println!("Generating keystore...");

    utils::execute_blocking(
        &keytool_path.as_os_str().to_string_lossy(),
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
