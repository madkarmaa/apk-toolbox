use crate::config::Config;
use crate::constants::errors;
use crate::utils;
use std::env;
use std::path::PathBuf;

pub fn compile(
    input: PathBuf,
    out_dir: Option<PathBuf>,
    keystore_alias: Option<String>,
    keystore_password: Option<String>,
) -> Result<(), String> {
    let out_dir =
        out_dir.unwrap_or_else(|| env::current_dir().unwrap_or_else(|_| PathBuf::from(".")));

    let keystore_path = Config::KeystorePath
        .get()
        .expect(errors::KEYSTORE_PATH_EXPECTED);

    let keystore_alias = keystore_alias
        .or_else(|| Config::KeystoreAlias.get())
        .ok_or_else(|| errors::KEYSTORE_ALIAS_NOT_FOUND.to_string())?;

    let keystore_password = keystore_password
        .or_else(|| Config::KeystorePassword.get())
        .ok_or_else(|| errors::KEYSTORE_PASSWORD_NOT_FOUND.to_string())?;

    println!(
        "Compiling {} to {}",
        input.to_string_lossy(),
        out_dir.to_string_lossy()
    );

    Ok(())
}

pub fn decompile(input: PathBuf, out_dir: Option<PathBuf>) -> Result<(), String> {
    let out_dir =
        out_dir.unwrap_or_else(|| env::current_dir().unwrap_or_else(|_| PathBuf::from(".")));

    println!(
        "Decompiling {} to {}",
        input.to_string_lossy(),
        out_dir.to_string_lossy()
    );

    Ok(())
}

pub fn keygen(
    keystore_alias: Option<String>,
    keystore_password: Option<String>,
) -> Result<(), String> {
    let keystore_path = Config::KeystorePath
        .get()
        .expect(errors::KEYSTORE_PATH_EXPECTED);

    let keystore_alias = keystore_alias
        .or_else(|| Config::KeystoreAlias.get())
        .ok_or_else(|| errors::KEYSTORE_ALIAS_NOT_FOUND.to_string())?;

    let keystore_password = keystore_password
        .or_else(|| Config::KeystorePassword.get())
        .ok_or_else(|| errors::KEYSTORE_PASSWORD_NOT_FOUND.to_string())?;

    let java_home = Config::JavaHome
        .get()
        .ok_or_else(|| errors::JAVA_HOME_NOT_CONFIGURED.to_string())?;

    let executable_name = if cfg!(windows) {
        "keytool.exe"
    } else {
        "keytool"
    };

    let keytool_path = PathBuf::from(java_home).join("bin").join(executable_name);

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
    )
    .map_err(|err| err.to_string())?;

    println!(
        "Key '{}' generated successfully at {}",
        keystore_alias, keystore_path
    );

    Ok(())
}
