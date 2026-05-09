use crate::cli::handlers::utils::java_bin_override;
use crate::config::Config;
use crate::utils;
use std::path::PathBuf;

fn merge_apks(input: &PathBuf) -> anyhow::Result<PathBuf> {
    let apkeditor_path = Config::ApkeditorPath.get()?;
    let output = input.with_extension("merged.apk");

    utils::execute_blocking(
        "java",
        java_bin_override("java"),
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

    let apktool_path = Config::ApktoolPath.get()?;

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
        "Decompiling {} to {}",
        input_file_name,
        out_dir.to_string_lossy()
    );

    let out_dir_str = out_dir.to_string_lossy();
    let input_str = input.to_string_lossy();

    let mut apktool_args = vec![
        "-jar",
        &apktool_path,
        "d",
        "-f",
        "-o",
        &out_dir_str,
        &input_str,
    ];

    let jobs_str;
    if let Some(jobs) = jobs {
        jobs_str = jobs.to_string();
        apktool_args.extend_from_slice(&["--jobs", &jobs_str]);
    }

    utils::execute_blocking("java", java_bin_override("java"), &apktool_args)?;

    println!("Decompiled successfully to {}", out_dir.to_string_lossy());

    Ok(())
}
