use crate::utils;
use serde_valid::validation;
use std::path::Path;

pub fn validate_java_home(path: &Option<String>) -> Result<(), validation::Error> {
    let Some(path) = path else { return Ok(()) };
    let p = Path::new(path);

    utils::ensure_exists(p).map_err(|e| validation::Error::Custom(e.to_string()))?;
    utils::ensure_directory(p).map_err(|e| validation::Error::Custom(e.to_string()))?;

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

    utils::ensure_exists(p).map_err(|e| validation::Error::Custom(e.to_string()))?;
    utils::ensure_has_extension(p, &["jar"])
        .map_err(|e| validation::Error::Custom(e.to_string()))?;

    Ok(())
}

pub fn validate_zipalign_path(path: &Option<String>) -> Result<(), validation::Error> {
    let Some(path) = path else { return Ok(()) };
    let p = Path::new(path);

    utils::ensure_exists(p).map_err(|e| validation::Error::Custom(e.to_string()))?;
    utils::ensure_file(p).map_err(|e| validation::Error::Custom(e.to_string()))?;

    if cfg!(windows) {
        utils::ensure_has_extension(p, &["exe"])
            .map_err(|e| validation::Error::Custom(e.to_string()))?;
    }

    Ok(())
}

pub fn validate_keystore_path(path: &Option<String>) -> Result<(), validation::Error> {
    let Some(path) = path else { return Ok(()) };
    let p = Path::new(path);

    utils::ensure_has_extension(p, &["jks"])
        .map_err(|e| validation::Error::Custom(e.to_string()))?;

    Ok(())
}
