use std::path::Path;

pub fn validate_java_path(path: &Option<String>) -> Result<(), serde_valid::validation::Error> {
    let Some(path) = path else { return Ok(()) };
    let p = Path::new(path);
    if !p.exists() {
        return Err(serde_valid::validation::Error::Custom(
            "Java path must exist".to_owned(),
        ));
    }
    if !p.is_dir() {
        return Err(serde_valid::validation::Error::Custom(
            "Java path must be a directory".to_owned(),
        ));
    }
    if path.ends_with('/') || path.ends_with('\\') {
        return Err(serde_valid::validation::Error::Custom(
            "Java path must not have a trailing slash".to_owned(),
        ));
    }
    if path.ends_with("/bin") || path.ends_with("\\bin") {
        return Err(serde_valid::validation::Error::Custom(
            "Java path must not end in /bin segment".to_owned(),
        ));
    }
    Ok(())
}

pub fn validate_jar_path(path: &Option<String>) -> Result<(), serde_valid::validation::Error> {
    let Some(path) = path else { return Ok(()) };
    let p = Path::new(path);
    if !p.exists() {
        return Err(serde_valid::validation::Error::Custom(
            "Path must exist".to_owned(),
        ));
    }
    if !p.is_file() {
        return Err(serde_valid::validation::Error::Custom(
            "Path must be a file".to_owned(),
        ));
    }
    if !path.ends_with(".jar") {
        return Err(serde_valid::validation::Error::Custom(
            "Path must end in .jar".to_owned(),
        ));
    }
    Ok(())
}

pub fn validate_zipalign_path(path: &Option<String>) -> Result<(), serde_valid::validation::Error> {
    let Some(path) = path else { return Ok(()) };
    let p = Path::new(path);
    if !p.exists() {
        return Err(serde_valid::validation::Error::Custom(
            "Path must exist".to_owned(),
        ));
    }
    if !p.is_file() {
        return Err(serde_valid::validation::Error::Custom(
            "Path must be a file".to_owned(),
        ));
    }
    if cfg!(windows) && !path.ends_with(".exe") {
        return Err(serde_valid::validation::Error::Custom(
            "Path must end in .exe on Windows".to_owned(),
        ));
    }
    Ok(())
}

pub fn validate_keystore_path(path: &Option<String>) -> Result<(), serde_valid::validation::Error> {
    let Some(path) = path else { return Ok(()) };
    if !path.ends_with(".jks") {
        return Err(serde_valid::validation::Error::Custom(
            "Path must end in .jks".to_owned(),
        ));
    }
    Ok(())
}
