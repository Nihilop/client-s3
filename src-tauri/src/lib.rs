mod auth;
mod error;
mod s3;
mod state;

use state::AppState;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_process::init())
        .manage(AppState::new())
        .invoke_handler(tauri::generate_handler![
            // Auth / Connection management
            auth::commands::save_connection,
            auth::commands::update_connection,
            auth::commands::list_connections,
            auth::commands::delete_connection,
            auth::commands::connect,
            auth::commands::disconnect,
            auth::commands::get_active_connection,
            auth::commands::test_connection,
            auth::commands::connect_direct,
            auth::commands::export_profiles,
            auth::commands::import_profiles,
            // S3 operations
            s3::commands::create_bucket,
            s3::commands::delete_bucket,
            s3::commands::list_buckets,
            s3::commands::list_objects,
            s3::commands::head_object,
            s3::commands::delete_object,
            s3::commands::delete_objects,
            s3::commands::create_folder,
            s3::commands::copy_object,
            s3::commands::move_object,
            s3::commands::search_objects,
            s3::commands::get_presigned_url,
            s3::commands::upload_file,
            s3::commands::download_file,
            s3::commands::get_object_preview,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
