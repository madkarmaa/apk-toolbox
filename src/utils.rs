use std::env;
use std::io;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

pub fn root_dir() -> PathBuf {
    let mut path: PathBuf;

    if cfg!(debug_assertions) {
        path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    } else {
        path = env::current_exe().unwrap_or_else(|_| PathBuf::from("."));
        path.pop();
    }

    path
}

pub fn execute_blocking(program: &str, args: &[&str]) -> io::Result<()> {
    let output = Command::new(program)
        .args(args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()?;

    if !output.status.success() {
        let mut err = String::from_utf8_lossy(&output.stderr);
        if err.is_empty() {
            err = String::from_utf8_lossy(&output.stdout);
        }

        return Err(io::Error::new(
            io::ErrorKind::Other,
            format!("{}", err.trim()),
        ));
    }

    Ok(())
}

pub fn assert_is_file(path: &Path, should_exist: bool) -> Result<(), String> {
    if should_exist && !path.exists() {
        return Err(format!("File not found at {}", path.to_string_lossy()));
    }

    if !path.is_file() {
        return Err(format!("Expected {} to be a file", path.to_string_lossy()));
    }

    Ok(())
}

pub fn assert_is_directory(path: &Path, should_exist: bool) -> Result<(), String> {
    if should_exist && !path.exists() {
        return Err(format!("Directory not found at {}", path.to_string_lossy()));
    }

    if !path.is_dir() {
        return Err(format!(
            "Expected {} to be a directory",
            path.to_string_lossy()
        ));
    }

    Ok(())
}

pub fn assert_has_extension(
    path: &Path,
    extensions: &[&str],
    should_exist: bool,
) -> Result<(), String> {
    assert_is_file(path, should_exist)?;

    if let Some(ext) = path.extension().and_then(|ext| ext.to_str()) {
        if extensions.contains(&ext) {
            return Ok(());
        }
    }

    Err(format!(
        "Expected {} to have one of the extensions: {}",
        path.to_string_lossy(),
        extensions.join(", ")
    ))
}
