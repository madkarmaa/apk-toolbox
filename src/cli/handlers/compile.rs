use crate::cli::handlers::utils::java_bin_override;
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

    let input_dir_str = input_dir.to_string_lossy();

    let out_file = out_file.unwrap_or_else(|| {
        utils::current_dir()
            .join(input_dir.file_name().unwrap_or_default())
            .with_extension("apk")
    });
    let out_file_str = out_file.to_string_lossy();

    utils::ensure_has_extension(&out_file, &["apk"])?;

    let apktool_path = Config::ApktoolPath.get()?;

    let build_tools_path = Config::BuildToolsPath.get()?;
    let build_tools_dir = Path::new(&build_tools_path);

    let keystore_path = Config::KeystorePath.get()?;

    let keystore_alias = keystore_alias
        .map(Ok)
        .unwrap_or_else(|| Config::KeystoreAlias.get())?;

    let keystore_password = keystore_password
        .map(Ok)
        .unwrap_or_else(|| Config::KeystorePassword.get())?;

    let zipalign_path = if cfg!(windows) {
        build_tools_dir.join("zipalign.exe")
    } else {
        build_tools_dir.join("zipalign")
    };
    let apksigner_path = build_tools_dir.join("lib").join("apksigner.jar");

    utils::ensure_exists(Path::new(&keystore_path))
        .map_err(|_| errors::AppError::KeystoreNotFound(keystore_path.to_string()))?;

    println!("Compiling {} to {}", input_dir_str, out_file_str);

    let unsigned_apk = out_file.with_extension("unsigned.apk");
    let unsigned_apk_str = unsigned_apk.to_string_lossy();

    let aligned_apk = out_file.with_extension("aligned.apk");
    let aligned_apk_str = aligned_apk.to_string_lossy();

    let mut apktool_args = vec![
        "-jar",
        &apktool_path,
        "b",
        "-f",
        "-o",
        &unsigned_apk_str,
        &input_dir_str,
    ];

    let jobs_str;
    if let Some(jobs) = jobs {
        jobs_str = jobs.to_string();
        apktool_args.extend_from_slice(&["--jobs", &jobs_str]);
    }

    utils::execute_blocking("java", java_bin_override("java"), &apktool_args)?;

    println!("Compiled successfully to {}", unsigned_apk_str);
    println!("Aligning APK with zipalign");

    utils::execute_blocking(
        "zipalign",
        Some(zipalign_path.clone()),
        &["-f", "-v", "4", &unsigned_apk_str, &aligned_apk_str],
    )?;

    println!("Aligned APK created at {}", aligned_apk_str);
    println!("Signing APK with apksigner");

    utils::execute_blocking(
        "java",
        java_bin_override("java"),
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
            &out_file_str,
            &aligned_apk_str,
        ],
    )?;

    println!("Signed APK created at {}", out_file_str);

    Ok(())
}
