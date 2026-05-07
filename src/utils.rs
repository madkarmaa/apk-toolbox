use crate::constants::errors::AppError;
use std::env;
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

pub fn current_dir() -> PathBuf {
    env::current_dir().unwrap_or_else(|_| PathBuf::from("."))
}

fn cmd_to_string(cmd: &Command) -> String {
    let prog = cmd.get_program().to_string_lossy();
    let args: Vec<_> = cmd.get_args().map(|a| a.to_string_lossy()).collect();

    std::iter::once(prog)
        .chain(args)
        .collect::<Vec<_>>()
        .join(" ")
}

pub fn execute_blocking(program: &str, args: &[&str]) -> Result<(), AppError> {
    let mut cmd = Command::new(program);
    cmd.args(args).stdout(Stdio::piped()).stderr(Stdio::piped());

    let output = cmd.output()?;
    if !output.status.success() {
        let mut err = String::from_utf8_lossy(&output.stderr);
        if err.is_empty() {
            err = String::from_utf8_lossy(&output.stdout);
        }

        return Err(AppError::ExecutionFailed(format!(
            "{}\n\n{}",
            cmd_to_string(&cmd),
            err.trim()
        )));
    }

    Ok(())
}

pub fn ensure_exists(path: &Path) -> Result<(), AppError> {
    if !path.exists() {
        return Err(AppError::PathNotFound(path.to_string_lossy().to_string()));
    }
    Ok(())
}

pub fn ensure_file(path: &Path) -> Result<(), AppError> {
    if path.exists() && !path.is_file() {
        return Err(AppError::ExpectedFile(path.to_string_lossy().to_string()));
    }
    Ok(())
}

pub fn ensure_directory(path: &Path) -> Result<(), AppError> {
    if path.exists() && !path.is_dir() {
        return Err(AppError::ExpectedDirectory(
            path.to_string_lossy().to_string(),
        ));
    }
    Ok(())
}

pub fn ensure_has_extension(path: &Path, extensions: &[&str]) -> Result<(), AppError> {
    ensure_file(path)?;

    if let Some(ext) = path.extension().and_then(|ext| ext.to_str()) {
        if extensions.contains(&ext) {
            return Ok(());
        }
    }

    Err(AppError::ExpectedExtension(
        path.to_string_lossy().to_string(),
        extensions.join(", "),
    ))
}

pub fn format_validation_error(error: &serde_valid::validation::Errors) -> String {
    let json_str = error.to_string();
    if let Ok(v) = serde_json::from_str::<serde_json::Value>(&json_str) {
        let mut messages = Vec::new();
        extract_errors(&v, &mut messages);
        if !messages.is_empty() {
            return messages.join(", ");
        }
    }
    json_str
}

pub fn extract_errors(value: &serde_json::Value, messages: &mut Vec<String>) {
    match value {
        serde_json::Value::Object(map) => {
            if let Some(serde_json::Value::Array(errors)) = map.get("errors") {
                for error in errors {
                    if let Some(s) = error.as_str() {
                        messages.push(s.to_string());
                    }
                }
            }
            if let Some(serde_json::Value::Object(props)) = map.get("properties") {
                for (_, val) in props {
                    extract_errors(val, messages);
                }
            }
        }
        _ => {}
    }
}
