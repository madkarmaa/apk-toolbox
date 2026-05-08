use crate::cli::handlers::utils::get_java_bin;
use crate::config::Config;
use crate::constants::errors;
use crate::utils;
use std::path::{Path, PathBuf};

pub fn compile(
    input_dir: PathBuf,
    out_file: Option<PathBuf>,
    keystore_alias: Option<String>,
    keystore_password: Option<String>,
    jobs: Option<usize>,
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

    let build_tools_path = Config::BuildToolsPath
        .get()?
        .ok_or_else(|| errors::AppError::BuildToolsPathNotConfigured)?;

    let build_tools_dir = Path::new(&build_tools_path);
    let zipalign_path = if cfg!(windows) {
        build_tools_dir.join("zipalign.exe")
    } else {
        build_tools_dir.join("zipalign")
    };
    let apksigner_path = build_tools_dir.join("lib").join("apksigner.jar");

    let keystore_path = Config::KeystorePath
        .get()?
        .ok_or_else(|| errors::AppError::KeystorePathExpected)?;

    utils::ensure_exists(Path::new(&keystore_path))
        .map_err(|_| errors::AppError::KeystoreNotFound(keystore_path.to_string()))?;

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
        &zipalign_path.to_string_lossy(),
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
            "-jar",
            &apksigner_path.to_string_lossy(),
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
