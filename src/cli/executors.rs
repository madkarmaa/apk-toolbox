use crate::config::Config;
use crate::constants::errors;
use crate::utils;
use std::path::PathBuf;

pub fn compile(
    input_dir: PathBuf,
    out_file: Option<PathBuf>,
    keystore_alias: Option<String>,
    keystore_password: Option<String>,
    jobs: Option<usize>,
) -> Result<(), String> {
    utils::assert_is_directory(&input_dir, true)?;

    let out_file = out_file.unwrap_or_else(|| {
        utils::current_dir()
            .join(input_dir.file_name().unwrap_or_default())
            .with_extension("apk")
    });

    utils::assert_has_extension(&out_file, &["apk"], false)?;

    let jobs = jobs.unwrap_or_else(|| num_cpus::get());

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
        input_dir.to_string_lossy(),
        out_file.to_string_lossy()
    );

    Ok(())
}

fn merge_apks(input: &PathBuf) -> Result<PathBuf, String> {
    let java_home = Config::JavaHome
        .get()
        .ok_or_else(|| errors::JAVA_HOME_NOT_CONFIGURED.to_string())?;

    let java_executable_name = if cfg!(windows) { "java.exe" } else { "java" };

    let java_path = PathBuf::from(java_home)
        .join("bin")
        .join(java_executable_name);

    let apkeditor_path = Config::ApkeditorPath
        .get()
        .ok_or_else(|| errors::APKEDITOR_PATH_NOT_CONFIGURED.to_string())?;

    let output = input.with_extension("merged.apk");

    utils::execute_blocking(
        &java_path.to_string_lossy(),
        &[
            "-jar",
            &apkeditor_path,
            "m",
            "-f",
            "-clean-meta",
            "-extractNativeLibs",
            "true",
            "-i",
            &input.to_string_lossy(),
            "-o",
            &output.to_string_lossy(),
        ],
    )
    .map_err(|err| err.to_string())?;

    Ok(output)
}

pub fn decompile(
    mut input: PathBuf,
    out_dir: Option<PathBuf>,
    jobs: Option<usize>,
) -> Result<(), String> {
    utils::assert_has_extension(&input, &["apk", "xapk", "apks"], true)?;

    let out_dir =
        out_dir.unwrap_or_else(|| utils::current_dir().join(input.file_stem().unwrap_or_default()));

    utils::assert_is_directory(&out_dir, false)?;

    let jobs = jobs.unwrap_or_else(|| num_cpus::get());

    let java_home = Config::JavaHome
        .get()
        .ok_or_else(|| errors::JAVA_HOME_NOT_CONFIGURED.to_string())?;

    let java_executable_name = if cfg!(windows) { "java.exe" } else { "java" };

    let java_path = PathBuf::from(java_home)
        .join("bin")
        .join(java_executable_name);

    let apktool_path = Config::ApktoolPath
        .get()
        .ok_or_else(|| errors::APKTOOL_PATH_NOT_CONFIGURED.to_string())?;

    let input_extension = input
        .extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or_default();

    if input_extension != "apk" {
        println!("Detected split APK to merge");
        input = merge_apks(&input)?;
        println!("Merged APK created at {}", input.to_string_lossy());
    }

    let input_file_name = input
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or_default();

    println!(
        "Decompiling {} to {} with {} parallel jobs",
        input_file_name,
        out_dir.to_string_lossy(),
        jobs
    );

    utils::execute_blocking(
        &java_path.to_string_lossy(),
        &[
            "-jar",
            &apktool_path,
            "d",
            "-f",
            "--jobs",
            &jobs.to_string(),
            "-o",
            &out_dir.to_string_lossy(),
            &input.to_string_lossy(),
        ],
    )
    .map_err(|err| err.to_string())?;

    println!("Decompiled successfully to {}", out_dir.to_string_lossy());

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
