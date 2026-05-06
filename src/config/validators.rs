use crate::utils;
use serde_valid::validation;
use std::path::Path;

pub fn validate_java_home(path: &Option<String>) -> Result<(), validation::Error> {
    let Some(path) = path else { return Ok(()) };
    let p = Path::new(path);

    utils::assert_is_directory(p, true).map_err(validation::Error::Custom)?;

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

    utils::assert_has_extension(p, &["jar"], true).map_err(validation::Error::Custom)?;

    Ok(())
}

pub fn validate_zipalign_path(path: &Option<String>) -> Result<(), validation::Error> {
    let Some(path) = path else { return Ok(()) };
    let p = Path::new(path);

    utils::assert_is_file(p, true).map_err(validation::Error::Custom)?;

    if cfg!(windows) {
        utils::assert_has_extension(p, &["exe"], true).map_err(validation::Error::Custom)?;
    }

    Ok(())
}

pub fn validate_keystore_path(path: &Option<String>) -> Result<(), validation::Error> {
    let Some(path) = path else { return Ok(()) };
    let p = Path::new(path);

    utils::assert_has_extension(p, &["jks"], false).map_err(validation::Error::Custom)?;

    Ok(())
}
