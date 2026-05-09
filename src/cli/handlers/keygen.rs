use crate::cli::handlers::utils::java_bin_override;
use crate::config::Config;
use crate::utils;

pub fn keygen(
    keystore_alias: Option<String>,
    keystore_password: Option<String>,
) -> anyhow::Result<()> {
    let keystore_path = Config::KeystorePath.get()?;

    let keystore_alias = keystore_alias
        .map(Ok)
        .unwrap_or_else(|| Config::KeystoreAlias.get())?;

    let keystore_password = keystore_password
        .map(Ok)
        .unwrap_or_else(|| Config::KeystorePassword.get())?;

    println!("Generating key '{}'", keystore_alias);

    utils::execute_blocking(
        "keytool",
        java_bin_override("keytool"),
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
