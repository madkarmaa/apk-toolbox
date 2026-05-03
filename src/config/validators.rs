use serde_valid::validation;
use std::path::Path;

pub fn validate_java_home(path: &Option<String>) -> Result<(), validation::Error> {
    let Some(path) = path else { return Ok(()) };
    let p = Path::new(path);

    if !p.exists() {
        return Err(validation::Error::Custom(
            "Java home must exist".to_string(),
        ));
    }

    if !p.is_dir() {
        return Err(validation::Error::Custom(
            "Java home must be a directory".to_string(),
        ));
    }

    if path.ends_with("/bin") || path.ends_with("\\bin") {
        return Err(validation::Error::Custom(
            "Java home must not end in /bin segment".to_string(),
        ));
    }

    Ok(())
}

pub fn validate_jar_path(path: &Option<String>) -> Result<(), validation::Error> {
    let Some(path) = path else { return Ok(()) };
    let p = Path::new(path);

    if !p.exists() {
        return Err(validation::Error::Custom("Path must exist".to_string()));
    }

    if !p.is_file() {
        return Err(validation::Error::Custom("Path must be a file".to_string()));
    }

    if !path.ends_with(".jar") {
        return Err(validation::Error::Custom(
            "Path must end in .jar".to_string(),
        ));
    }

    Ok(())
}

pub fn validate_zipalign_path(path: &Option<String>) -> Result<(), validation::Error> {
    let Some(path) = path else { return Ok(()) };
    let p = Path::new(path);

    if !p.exists() {
        return Err(validation::Error::Custom("Path must exist".to_string()));
    }

    if !p.is_file() {
        return Err(validation::Error::Custom("Path must be a file".to_string()));
    }

    if cfg!(windows) && !path.ends_with(".exe") {
        return Err(validation::Error::Custom(
            "Path must end in .exe on Windows".to_string(),
        ));
    }

    Ok(())
}

pub fn validate_keystore_path(path: &Option<String>) -> Result<(), validation::Error> {
    let Some(path) = path else { return Ok(()) };

    if !path.ends_with(".jks") {
        return Err(validation::Error::Custom(
            "Path must end in .jks".to_string(),
        ));
    }

    Ok(())
}
