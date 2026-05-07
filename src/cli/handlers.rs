use crate::config::Config;
use crate::constants::errors;
use crate::utils;
use std::path::PathBuf;

fn get_java_bin(name: &str) -> Result<PathBuf, errors::AppError> {
    let java_home = Config::JavaHome
        .get()?
        .ok_or_else(|| errors::AppError::JavaHomeNotConfigured)?;

    let executable_name = if cfg!(windows) {
        format!("{}.exe", name)
    } else {
        name.to_string()
    };

    Ok(PathBuf::from(java_home).join("bin").join(executable_name))
}

pub fn compile(
    input_dir: PathBuf,
    out_file: Option<PathBuf>,
    keystore_alias: Option<String>,
    keystore_password: Option<String>,
    jobs: Option<usize>,
    jvm_heap: String,
) -> anyhow::Result<()> {
    utils::ensure_exists(&input_dir)?;
    utils::ensure_directory(&input_dir)?;

    let out_file = out_file.unwrap_or_else(|| {
        utils::current_dir()
            .join(input_dir.file_name().unwrap_or_default())
            .with_extension("apk")
    });

    utils::ensure_has_extension(&out_file, &["apk"])?;

    let jobs = jobs.unwrap_or_else(|| num_cpus::get());

    let java_path = get_java_bin("java")?;

    let apktool_path = Config::ApktoolPath
        .get()?
        .ok_or_else(|| errors::AppError::ApktoolPathNotConfigured)?;

    let zipalign_path = Config::ZipalignPath
        .get()?
        .ok_or_else(|| errors::AppError::ZipalignPathNotConfigured)?;

    let apksigner_path = Config::ApksignerPath
        .get()?
        .ok_or_else(|| errors::AppError::ApksignerPathNotConfigured)?;

    let keystore_path = Config::KeystorePath
        .get()?
        .ok_or_else(|| errors::AppError::KeystorePathExpected)?;

    let keystore_alias = keystore_alias
        .or_else(|| Config::KeystoreAlias.get().unwrap_or_default())
        .ok_or_else(|| errors::AppError::KeystoreAliasNotFound)?;

    let keystore_password = keystore_password
        .or_else(|| Config::KeystorePassword.get().unwrap_or_default())
        .ok_or_else(|| errors::AppError::KeystorePasswordNotFound)?;

    println!(
        "Compiling {} to {} with {} parallel jobs",
        input_dir.to_string_lossy(),
        out_file.to_string_lossy(),
        jobs
    );

    let unsigned_apk = out_file.with_extension("unsigned.apk");
    let aligned_apk = out_file.with_extension("aligned.apk");

    utils::execute_blocking(
        &java_path.to_string_lossy(),
        &[
            &format!("-Xmx{}", jvm_heap),
            "-jar",
            &apktool_path,
            "b",
            "-f",
            "--jobs",
            &jobs.to_string(),
            &input_dir.to_string_lossy(),
            "-o",
            &unsigned_apk.to_string_lossy(),
        ],
    )?;

    println!(
        "Compiled successfully to {}",
        unsigned_apk.to_string_lossy()
    );
    println!("Aligning APK with zipalign");

    utils::execute_blocking(
        &zipalign_path,
        &[
            "-f",
            "-v",
            "4",
            &unsigned_apk.to_string_lossy(),
            &aligned_apk.to_string_lossy(),
        ],
    )?;

    println!("Aligned APK created at {}", aligned_apk.to_string_lossy());
    println!("Signing APK with apksigner");

    utils::execute_blocking(
        &java_path.to_string_lossy(),
        &[
            &format!("-Xmx{}", jvm_heap),
            "-jar",
            &apksigner_path,
            "sign",
            "--ks",
            &keystore_path,
            "--ks-key-alias",
            &keystore_alias,
            "--ks-pass",
            &format!("pass:{}", keystore_password),
            "--key-pass",
            &format!("pass:{}", keystore_password),
            "--out",
            &out_file.to_string_lossy(),
            &aligned_apk.to_string_lossy(),
        ],
    )?;

    println!("Signed APK created at {}", out_file.to_string_lossy());

    Ok(())
}

fn merge_apks(input: &PathBuf, jvm_heap: &String) -> anyhow::Result<PathBuf> {
    let java_path = get_java_bin("java")?;

    let apkeditor_path = Config::ApkeditorPath
        .get()?
        .ok_or_else(|| errors::AppError::ApkeditorPathNotConfigured)?;

    let output = input.with_extension("merged.apk");

    utils::execute_blocking(
        &java_path.to_string_lossy(),
        &[
            &format!("-Xmx{}", jvm_heap),
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
    )?;

    Ok(output)
}

pub fn decompile(
    mut input: PathBuf,
    out_dir: Option<PathBuf>,
    jobs: Option<usize>,
    jvm_heap: String,
) -> anyhow::Result<()> {
    utils::ensure_exists(&input)?;
    utils::ensure_has_extension(&input, &["apk", "xapk", "apks"])?;

    let out_dir =
        out_dir.unwrap_or_else(|| utils::current_dir().join(input.file_stem().unwrap_or_default()));

    utils::ensure_directory(&out_dir)?;

    let jobs = jobs.unwrap_or_else(|| num_cpus::get());

    let java_path = get_java_bin("java")?;

    let apktool_path = Config::ApktoolPath
        .get()?
        .ok_or_else(|| errors::AppError::ApktoolPathNotConfigured)?;

    let input_extension = input
        .extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or_default();

    if input_extension != "apk" {
        println!("Detected split APK to merge");
        input = merge_apks(&input, &jvm_heap)?;
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
            &format!("-Xmx{}", jvm_heap),
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
    )?;

    println!("Decompiled successfully to {}", out_dir.to_string_lossy());

    Ok(())
}

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
