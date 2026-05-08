use crate::cli::handlers::utils::get_java_bin;
use crate::config::Config;
use crate::constants::errors;
use crate::utils;

pub fn keygen(
    keystore_alias: Option<String>,
    keystore_password: Option<String>,
) -> anyhow::Result<()> {
    let keystore_path = Config::KeystorePath
        .get()?
        .ok_or_else(|| errors::AppError::KeystorePathExpected)?;

    let keystore_alias = keystore_alias
        .or_else(|| Config::KeystoreAlias.get().unwrap_or_default())
        .ok_or_else(|| errors::AppError::KeystoreAliasNotFound)?;

    let keystore_password = keystore_password
        .or_else(|| Config::KeystorePassword.get().unwrap_or_default())
        .ok_or_else(|| errors::AppError::KeystorePasswordNotFound)?;

    let keytool_path = get_java_bin("keytool")?;

    println!("Generating key '{}'", keystore_alias);

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
    )?;
    println!(
        "Key '{}' generated successfully at {}",
        keystore_alias, keystore_path
    );

    Ok(())
}
