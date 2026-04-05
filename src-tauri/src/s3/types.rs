use serde::{Deserialize, Serialize};

/// S3 connection profile stored in keyring
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionProfile {
    pub id: String,
    pub name: String,
    pub endpoint: String,
    pub region: String,
    pub access_key_id: String,
    pub secret_access_key: String,
    pub path_style: bool,
}

/// Safe version sent to the frontend (no secret)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionInfo {
    pub id: String,
    pub name: String,
    pub endpoint: String,
    pub region: String,
    pub access_key_id: String,
    pub path_style: bool,
}

impl From<&ConnectionProfile> for ConnectionInfo {
    fn from(p: &ConnectionProfile) -> Self {
        Self {
            id: p.id.clone(),
            name: p.name.clone(),
            endpoint: p.endpoint.clone(),
            region: p.region.clone(),
            access_key_id: p.access_key_id.clone(),
            path_style: p.path_style,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BucketInfo {
    pub name: String,
    pub creation_date: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObjectInfo {
    pub key: String,
    pub display_name: String,
    pub size: i64,
    pub last_modified: Option<String>,
    pub etag: Option<String>,
    pub storage_class: Option<String>,
    pub is_folder: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObjectDetails {
    pub key: String,
    pub size: i64,
    pub last_modified: Option<String>,
    pub etag: Option<String>,
    pub content_type: Option<String>,
    pub storage_class: Option<String>,
    pub metadata: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListObjectsResult {
    pub objects: Vec<ObjectInfo>,
    pub common_prefixes: Vec<String>,
    pub next_continuation_token: Option<String>,
    pub is_truncated: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferProgress {
    pub id: String,
    pub file_name: String,
    pub bytes_transferred: u64,
    pub total_bytes: u64,
    pub status: TransferStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TransferStatus {
    Pending,
    InProgress,
    Completed,
    Failed(String),
    Cancelled,
}

/// Input for creating/updating a connection (includes secret)
#[derive(Debug, Clone, Deserialize)]
pub struct ConnectionInput {
    pub name: String,
    pub endpoint: String,
    pub region: String,
    pub access_key_id: String,
    pub secret_access_key: String,
    pub path_style: bool,
}

/// Exportable profile (includes secret, for export/import)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportableProfile {
    pub name: String,
    pub endpoint: String,
    pub region: String,
    pub access_key_id: String,
    pub secret_access_key: String,
    pub path_style: bool,
}

impl From<&ConnectionProfile> for ExportableProfile {
    fn from(p: &ConnectionProfile) -> Self {
        Self {
            name: p.name.clone(),
            endpoint: p.endpoint.clone(),
            region: p.region.clone(),
            access_key_id: p.access_key_id.clone(),
            secret_access_key: p.secret_access_key.clone(),
            path_style: p.path_style,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrefixSize {
    pub total_size: u64,
    pub object_count: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObjectVersion {
    pub version_id: Option<String>,
    pub is_latest: bool,
    pub last_modified: Option<String>,
    pub size: i64,
    pub etag: Option<String>,
    pub is_delete_marker: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BucketVersioningStatus {
    pub enabled: bool,
    pub mfa_delete: bool,
}

impl From<ExportableProfile> for ConnectionInput {
    fn from(p: ExportableProfile) -> Self {
        Self {
            name: p.name,
            endpoint: p.endpoint,
            region: p.region,
            access_key_id: p.access_key_id,
            secret_access_key: p.secret_access_key,
            path_style: p.path_style,
        }
    }
}
