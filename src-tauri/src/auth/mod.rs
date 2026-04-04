pub mod commands;

use crate::error::{AppError, AppResult};
use crate::s3::types::{ConnectionInfo, ConnectionInput, ConnectionProfile};
use keyring::Entry;
use uuid::Uuid;

const KEYRING_SERVICE: &str = "client-s3";

/// Derive a keyring entry key from a profile ID
fn profile_entry(profile_id: &str) -> AppResult<Entry> {
    Entry::new(KEYRING_SERVICE, profile_id).map_err(AppError::from)
}

/// Index entry that stores the list of profile IDs
fn index_entry() -> AppResult<Entry> {
    Entry::new(KEYRING_SERVICE, "__profile_index__").map_err(AppError::from)
}

/// Get all stored profile IDs
fn get_profile_ids() -> AppResult<Vec<String>> {
    let entry = index_entry()?;
    match entry.get_password() {
        Ok(data) => {
            let ids: Vec<String> = serde_json::from_str(&data)?;
            Ok(ids)
        }
        Err(keyring::Error::NoEntry) => Ok(Vec::new()),
        Err(e) => Err(AppError::from(e)),
    }
}

/// Save the profile ID index
fn save_profile_ids(ids: &[String]) -> AppResult<()> {
    let entry = index_entry()?;
    let data = serde_json::to_string(ids)?;
    entry.set_password(&data).map_err(AppError::from)
}

/// Save a connection profile securely in the OS keyring
pub fn save_profile(input: ConnectionInput) -> AppResult<ConnectionInfo> {
    validate_connection_input(&input)?;

    let id = Uuid::new_v4().to_string();
    let profile = ConnectionProfile {
        id: id.clone(),
        name: input.name,
        endpoint: input.endpoint,
        region: input.region,
        access_key_id: input.access_key_id,
        secret_access_key: input.secret_access_key,
        path_style: input.path_style,
    };

    let entry = profile_entry(&id)?;
    let serialized = serde_json::to_string(&profile)?;
    entry.set_password(&serialized).map_err(AppError::from)?;

    // Add to index
    let mut ids = get_profile_ids()?;
    ids.push(id);
    save_profile_ids(&ids)?;

    Ok(ConnectionInfo::from(&profile))
}

/// Update an existing profile
pub fn update_profile(id: &str, input: ConnectionInput) -> AppResult<ConnectionInfo> {
    validate_connection_input(&input)?;

    // Verify it exists
    let _existing = get_profile(id)?;

    let profile = ConnectionProfile {
        id: id.to_string(),
        name: input.name,
        endpoint: input.endpoint,
        region: input.region,
        access_key_id: input.access_key_id,
        secret_access_key: input.secret_access_key,
        path_style: input.path_style,
    };

    let entry = profile_entry(id)?;
    let serialized = serde_json::to_string(&profile)?;
    entry.set_password(&serialized).map_err(AppError::from)?;

    Ok(ConnectionInfo::from(&profile))
}

/// Retrieve a full profile (including secret) — internal use only
pub fn get_profile(id: &str) -> AppResult<ConnectionProfile> {
    let entry = profile_entry(id)?;
    match entry.get_password() {
        Ok(data) => {
            let profile: ConnectionProfile = serde_json::from_str(&data)?;
            Ok(profile)
        }
        Err(keyring::Error::NoEntry) => {
            Err(AppError::Auth(format!("Profile '{}' not found", id)))
        }
        Err(e) => Err(AppError::from(e)),
    }
}

/// List all saved profiles (safe info only, no secrets)
pub fn list_profiles() -> AppResult<Vec<ConnectionInfo>> {
    let ids = get_profile_ids()?;
    let mut profiles = Vec::new();
    for id in &ids {
        match get_profile(id) {
            Ok(p) => profiles.push(ConnectionInfo::from(&p)),
            Err(_) => {
                // Profile entry missing/corrupted — skip it
                log::warn!("Skipping corrupted profile entry: {}", id);
            }
        }
    }
    Ok(profiles)
}

/// Delete a profile from keyring
pub fn delete_profile(id: &str) -> AppResult<()> {
    let entry = profile_entry(id)?;
    match entry.delete_credential() {
        Ok(_) | Err(keyring::Error::NoEntry) => {}
        Err(e) => return Err(AppError::from(e)),
    }

    // Remove from index
    let mut ids = get_profile_ids()?;
    ids.retain(|i| i != id);
    save_profile_ids(&ids)?;

    Ok(())
}

/// Validate connection input to prevent injection / bad data
fn validate_connection_input(input: &ConnectionInput) -> AppResult<()> {
    if input.name.trim().is_empty() {
        return Err(AppError::Validation("Connection name is required".into()));
    }
    if input.name.len() > 128 {
        return Err(AppError::Validation("Connection name too long".into()));
    }
    if input.endpoint.trim().is_empty() {
        return Err(AppError::Validation("Endpoint is required".into()));
    }
    // Basic endpoint validation — must look like a URL
    if !input.endpoint.starts_with("http://") && !input.endpoint.starts_with("https://") {
        return Err(AppError::Validation(
            "Endpoint must start with http:// or https://".into(),
        ));
    }
    if input.region.trim().is_empty() {
        return Err(AppError::Validation("Region is required".into()));
    }
    if input.access_key_id.trim().is_empty() {
        return Err(AppError::Validation("Access key ID is required".into()));
    }
    if input.secret_access_key.trim().is_empty() {
        return Err(AppError::Validation("Secret access key is required".into()));
    }
    Ok(())
}
