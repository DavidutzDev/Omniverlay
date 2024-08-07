use thiserror::Error;

pub type OmniverlayResult<T> = std::result::Result<T, OmniverlayError>;

#[derive(Error, Debug)]
pub enum OmniverlayError {
    #[error("Tauri error: {0}")]
    TauriError(#[from] tauri::Error),

    #[error("Backend initialization error: {0}")]
    BackendInitialization(String),

    #[error("Extension not found: {0}")]
    ExtensionNotFound(String),

    #[error("Extension load failed: {0}")]
    ExtensionLoadFailed(String),

    #[error("Serialization error: {0}")]
    SerdeJson(#[from] serde_json::Error),

    #[error("Lock error: {0}")]
    LockError(String),

    #[error("Serialization error: {0}")]
    SerializationError(String),

    #[error("Config not found: {0}")]
    ConfigNotFound(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Cannot find the app directory")]
    AppDirectory,

    #[error("Profile not found: {0}")]
    ProfileNotFound(String),

    #[error("No data found for: {0}")]
    DataNotFound(String),

    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

impl From<OmniverlayError> for String {
    fn from(e: OmniverlayError) -> Self {
        e.to_string()
    }
}