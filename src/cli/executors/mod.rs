use crate::config::Config;
use crate::utils;

pub fn keygen() -> Result<(), String> {
    println!("Generating keystore...");

    let keystore_path = Config::KeystorePath.get().ok_or_else(|| {
        "Keystore path not configured. Please configure it using the config command.".to_string()
    })?;
    println!("Keystore path: {}", keystore_path);

    let keystore_alias = Config::KeystoreAlias.get().ok_or_else(|| {
        "Keystore alias not configured. Please configure it using the config command.".to_string()
    })?;
    println!("Keystore alias: {}", keystore_alias);

    let keystore_password = Config::KeystorePassword.get().ok_or_else(|| {
        "Keystore password not configured. Please configure it using the config command."
            .to_string()
    })?;
    println!("Keystore password: {}", keystore_password);

    utils::execute_blocking(
        "keytool",
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
