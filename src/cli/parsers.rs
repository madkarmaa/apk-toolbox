pub fn trim_string(v: &str) -> Result<String, String> {
    Ok(v.trim().to_string())
}

pub fn validate_keystore_alias(value: &str) -> Result<String, String> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        Err("Keystore alias cannot be empty".to_string())
    } else {
        Ok(trimmed.to_string())
    }
}

pub fn validate_keystore_password(value: &str) -> Result<String, String> {
    let trimmed = value.trim();
    if trimmed.len() < 6 {
        Err("Keystore password must be at least 6 characters".to_string())
    } else {
        Ok(trimmed.to_string())
    }
}

pub fn validate_apktool_jobs(value: &str) -> Result<usize, String> {
    let jobs = value
        .parse::<usize>()
        .map_err(|_| "Jobs must be a positive integer".to_string())?;

    if jobs == 0 || jobs > 8 {
        return Err("Jobs must be between 1 and 8".to_string());
    }
    Ok(jobs)
}
