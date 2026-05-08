use crate::utils;
use serde_valid::validation;
use std::path::Path;

pub fn validate_java_home(path: &Option<String>) -> Result<(), validation::Error> {
    let Some(path) = path else { return Ok(()) };
    let p = Path::new(path);

    utils::ensure_exists(p).map_err(|e| validation::Error::Custom(e.to_string()))?;
    utils::ensure_directory(p).map_err(|e| validation::Error::Custom(e.to_string()))?;

    let trimmed_path = path.trim_end_matches(['/', '\\']);
    if trimmed_path.ends_with("/bin") || trimmed_path.ends_with("\\bin") {
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

pub fn validate_build_tools_path(path: &Option<String>) -> Result<(), validation::Error> {
    let Some(path) = path else { return Ok(()) };
    let p = Path::new(path);

    utils::ensure_exists(p).map_err(|e| validation::Error::Custom(e.to_string()))?;
    utils::ensure_directory(p).map_err(|e| validation::Error::Custom(e.to_string()))?;

    let zipalign_path = if cfg!(windows) {
        p.join("zipalign.exe")
    } else {
        p.join("zipalign")
    };

    utils::ensure_exists(&zipalign_path)
        .map_err(|_| validation::Error::Custom(format!("zipalign not found in {}", path)))?;

    let apksigner_path = p.join("lib").join("apksigner.jar");
    utils::ensure_exists(&apksigner_path)
        .map_err(|_| validation::Error::Custom(format!("apksigner.jar not found in {}/lib", path)))?;

    Ok(())
}

pub fn validate_keystore_path(path: &Option<String>) -> Result<(), validation::Error> {
    let Some(path) = path else { return Ok(()) };
    let p = Path::new(path);

    utils::ensure_has_extension(p, &["jks"])
        .map_err(|e| validation::Error::Custom(e.to_string()))?;

    Ok(())
}
