use crate::config::Config;
use std::path::PathBuf;

pub fn java_bin_override(name: &str) -> Option<PathBuf> {
    let java_home = Config::JavaHome.get().ok()?;
    let program = if cfg!(windows) {
        format!("{}.exe", name)
    } else {
        name.to_string()
    };
    Some(PathBuf::from(java_home).join("bin").join(program))
}
