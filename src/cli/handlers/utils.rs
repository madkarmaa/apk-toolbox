use crate::config::Config;
use crate::constants::errors;
use std::path::PathBuf;

pub fn get_java_bin(name: &str) -> Result<PathBuf, errors::AppError> {
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
