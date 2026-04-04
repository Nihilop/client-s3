use tokio::sync::RwLock;
use aws_sdk_s3::Client as S3Client;

use crate::s3::types::ConnectionProfile;

/// Thread-safe application state managed by Tauri
pub struct AppState {
    pub active_connection: RwLock<Option<ActiveSession>>,
}

pub struct ActiveSession {
    pub profile: ConnectionProfile,
    pub client: S3Client,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            active_connection: RwLock::new(None),
        }
    }
}
