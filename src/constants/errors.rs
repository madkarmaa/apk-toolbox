use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Keystore path should have a default value")]
    KeystorePathExpected,

    #[error(
        "Keystore alias not found. Please pass it with the -a flag or configure it using the config command."
    )]
    KeystoreAliasNotFound,

    #[error(
        "Keystore password not found. Please pass it with the -p flag or configure it using the config command."
    )]
    KeystorePasswordNotFound,

    #[error("Java home not configured. Please configure it using the config command.")]
    JavaHomeNotConfigured,

    #[error("Apktool path not configured. Please configure it using the config command.")]
    ApktoolPathNotConfigured,

    #[error("Apkeditor path not configured. Please configure it using the config command.")]
    ApkeditorPathNotConfigured,

    #[error("Zipalign path not configured. Please configure it using the config command.")]
    ZipalignPathNotConfigured,

    #[error("Apksigner path not configured. Please configure it using the config command.")]
    ApksignerPathNotConfigured,

    #[error("Path not found at {0}")]
    PathNotFound(String),

    #[error("Expected {0} to be a file")]
    ExpectedFile(String),

    #[error("Expected {0} to be a directory")]
    ExpectedDirectory(String),

    #[error("Expected {0} to have one of the extensions: {1}")]
    ExpectedExtension(String, String),

    #[error("Execution failed: {0}")]
    ExecutionFailed(String),

    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error("Config error: {0}")]
    Config(String),
}
