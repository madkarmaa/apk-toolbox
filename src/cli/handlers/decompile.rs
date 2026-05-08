use crate::cli::handlers::utils::get_java_bin;
use crate::config::Config;
use crate::constants::errors;
use crate::utils;
use std::path::PathBuf;

fn merge_apks(input: &PathBuf) -> anyhow::Result<PathBuf> {
    let java_path = get_java_bin("java")?;

    let apkeditor_path = Config::ApkeditorPath
        .get()?
        .ok_or_else(|| errors::AppError::ApkeditorPathNotConfigured)?;

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
    )?;

    Ok(output)
}

pub fn decompile(
    mut input: PathBuf,
    out_dir: Option<PathBuf>,
    jobs: Option<usize>,
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
    )?;

    println!("Decompiled successfully to {}", out_dir.to_string_lossy());

    Ok(())
}
