use tauri::State;

use crate::auth;
use crate::error::AppResult;
use crate::s3::types::{ConnectionInfo, ConnectionInput, ConnectionProfile, ExportableProfile};
use crate::state::AppState;
use crate::s3;

#[tauri::command]
pub async fn save_connection(input: ConnectionInput) -> AppResult<ConnectionInfo> {
    auth::save_profile(input)
}

#[tauri::command]
pub async fn update_connection(id: String, input: ConnectionInput) -> AppResult<ConnectionInfo> {
    auth::update_profile(&id, input)
}

#[tauri::command]
pub async fn list_connections() -> AppResult<Vec<ConnectionInfo>> {
    auth::list_profiles()
}

#[tauri::command]
pub async fn delete_connection(
    id: String,
    state: State<'_, AppState>,
) -> AppResult<()> {
    // Disconnect if this is the active connection
    let session = state.active_connection.read().await;
    if let Some(ref s) = *session {
        if s.profile.id == id {
            drop(session);
            let mut session = state.active_connection.write().await;
            *session = None;
        }
    }
    auth::delete_profile(&id)
}

#[tauri::command]
pub async fn connect(
    id: String,
    state: State<'_, AppState>,
) -> AppResult<ConnectionInfo> {
    let profile = auth::get_profile(&id)?;
    let client = s3::create_client(&profile).await;

    // Verify the connection actually works
    s3::list_buckets(&client).await?;

    let info = ConnectionInfo::from(&profile);
    let mut session = state.active_connection.write().await;
    *session = Some(crate::state::ActiveSession { profile, client });

    Ok(info)
}

#[tauri::command]
pub async fn disconnect(state: State<'_, AppState>) -> AppResult<()> {
    let mut session = state.active_connection.write().await;
    *session = None;
    Ok(())
}

#[tauri::command]
pub async fn get_active_connection(state: State<'_, AppState>) -> AppResult<Option<ConnectionInfo>> {
    let session = state.active_connection.read().await;
    Ok(session.as_ref().map(|s| ConnectionInfo::from(&s.profile)))
}

/// Connect directly without saving a profile
#[tauri::command]
pub async fn connect_direct(
    input: ConnectionInput,
    state: State<'_, AppState>,
) -> AppResult<ConnectionInfo> {
    let profile = ConnectionProfile {
        id: uuid::Uuid::new_v4().to_string(),
        name: input.name,
        endpoint: input.endpoint,
        region: input.region,
        access_key_id: input.access_key_id,
        secret_access_key: input.secret_access_key,
        path_style: input.path_style,
    };
    let client = s3::create_client(&profile).await;

    // Verify the connection actually works
    s3::list_buckets(&client).await?;

    let info = ConnectionInfo::from(&profile);
    let mut session = state.active_connection.write().await;
    *session = Some(crate::state::ActiveSession { profile, client });

    Ok(info)
}

/// Export all profiles (with secrets) as JSON
#[tauri::command]
pub async fn export_profiles() -> AppResult<Vec<ExportableProfile>> {
    let ids = auth::list_profiles()?;
    let mut profiles = Vec::new();
    // list_profiles returns ConnectionInfo, we need full profiles
    for info in &ids {
        if let Ok(profile) = auth::get_profile(&info.id) {
            profiles.push(ExportableProfile::from(&profile));
        }
    }
    Ok(profiles)
}

/// Import profiles from exported JSON
#[tauri::command]
pub async fn import_profiles(profiles: Vec<ExportableProfile>) -> AppResult<Vec<ConnectionInfo>> {
    let mut results = Vec::new();
    for p in profiles {
        let input = ConnectionInput::from(p);
        let info = auth::save_profile(input)?;
        results.push(info);
    }
    Ok(results)
}

#[tauri::command]
pub async fn test_connection(input: ConnectionInput) -> AppResult<Vec<String>> {
    let profile = ConnectionProfile {
        id: String::new(),
        name: input.name,
        endpoint: input.endpoint,
        region: input.region,
        access_key_id: input.access_key_id,
        secret_access_key: input.secret_access_key,
        path_style: input.path_style,
    };

    let client = s3::create_client(&profile).await;
    let buckets = s3::list_buckets(&client).await?;
    Ok(buckets.into_iter().map(|b| b.name).collect())
}
