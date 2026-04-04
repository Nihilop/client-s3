use serde::Serialize;

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("S3 error: {0}")]
    S3(String),

    #[error("Authentication error: {0}")]
    Auth(String),

    #[error("Keyring error: {0}")]
    Keyring(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    Serde(#[from] serde_json::Error),

    #[error("Invalid input: {0}")]
    Validation(String),

    #[error("Not connected: no active S3 session")]
    NotConnected,
}

impl Serialize for AppError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl From<keyring::Error> for AppError {
    fn from(e: keyring::Error) -> Self {
        AppError::Keyring(e.to_string())
    }
}

pub type AppResult<T> = Result<T, AppError>;
