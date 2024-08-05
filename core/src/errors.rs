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

    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

impl From<OmniverlayError> for String {
    fn from(e: OmniverlayError) -> Self {
        e.to_string()
    }
}