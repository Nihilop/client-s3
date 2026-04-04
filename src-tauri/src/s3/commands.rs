use std::path::PathBuf;
use tauri::{AppHandle, Emitter, State};
use tokio::io::AsyncWriteExt;

use crate::error::{AppError, AppResult};
use crate::s3;
use crate::s3::types::*;
use crate::state::AppState;

/// Helper: get S3 client from state or return error
async fn get_client(state: &State<'_, AppState>) -> AppResult<aws_sdk_s3::Client> {
    let session = state.active_connection.read().await;
    match session.as_ref() {
        Some(s) => Ok(s.client.clone()),
        None => Err(AppError::NotConnected),
    }
}

#[tauri::command]
pub async fn create_bucket(state: State<'_, AppState>, bucket: String) -> AppResult<()> {
    let client = get_client(&state).await?;
    s3::create_bucket(&client, &bucket).await
}

#[tauri::command]
pub async fn delete_bucket(state: State<'_, AppState>, bucket: String) -> AppResult<()> {
    let client = get_client(&state).await?;
    s3::delete_bucket(&client, &bucket).await
}

#[tauri::command]
pub async fn list_buckets(state: State<'_, AppState>) -> AppResult<Vec<BucketInfo>> {
    let client = get_client(&state).await?;
    s3::list_buckets(&client).await
}

#[tauri::command]
pub async fn list_objects(
    state: State<'_, AppState>,
    bucket: String,
    prefix: String,
    continuation_token: Option<String>,
    max_keys: Option<i32>,
) -> AppResult<ListObjectsResult> {
    let client = get_client(&state).await?;
    s3::list_objects(
        &client,
        &bucket,
        &prefix,
        continuation_token.as_deref(),
        max_keys.unwrap_or(1000),
    )
    .await
}

#[tauri::command]
pub async fn head_object(
    state: State<'_, AppState>,
    bucket: String,
    key: String,
) -> AppResult<ObjectDetails> {
    let client = get_client(&state).await?;
    s3::head_object(&client, &bucket, &key).await
}

#[tauri::command]
pub async fn delete_object(
    state: State<'_, AppState>,
    bucket: String,
    key: String,
) -> AppResult<()> {
    let client = get_client(&state).await?;
    s3::delete_object(&client, &bucket, &key).await
}

#[tauri::command]
pub async fn delete_objects(
    state: State<'_, AppState>,
    bucket: String,
    keys: Vec<String>,
) -> AppResult<Vec<String>> {
    let client = get_client(&state).await?;
    s3::delete_objects(&client, &bucket, &keys).await
}

#[tauri::command]
pub async fn create_folder(
    state: State<'_, AppState>,
    bucket: String,
    prefix: String,
) -> AppResult<()> {
    let client = get_client(&state).await?;
    s3::create_folder(&client, &bucket, &prefix).await
}

#[tauri::command]
pub async fn copy_object(
    state: State<'_, AppState>,
    source_bucket: String,
    source_key: String,
    dest_bucket: String,
    dest_key: String,
) -> AppResult<()> {
    let client = get_client(&state).await?;
    s3::copy_object(&client, &source_bucket, &source_key, &dest_bucket, &dest_key).await
}

#[tauri::command]
pub async fn move_object(
    state: State<'_, AppState>,
    source_bucket: String,
    source_key: String,
    dest_bucket: String,
    dest_key: String,
) -> AppResult<()> {
    let client = get_client(&state).await?;
    s3::copy_object(&client, &source_bucket, &source_key, &dest_bucket, &dest_key).await?;
    s3::delete_object(&client, &source_bucket, &source_key).await
}

#[tauri::command]
pub async fn get_presigned_url(
    state: State<'_, AppState>,
    bucket: String,
    key: String,
    expires_in_secs: Option<u64>,
) -> AppResult<String> {
    let client = get_client(&state).await?;
    s3::get_presigned_url(&client, &bucket, &key, expires_in_secs.unwrap_or(3600)).await
}

#[tauri::command]
pub async fn search_objects(
    state: State<'_, AppState>,
    bucket: String,
    query: String,
    max_results: Option<usize>,
) -> AppResult<Vec<ObjectInfo>> {
    let client = get_client(&state).await?;
    s3::search_objects(&client, &bucket, &query, max_results.unwrap_or(50)).await
}

#[tauri::command]
pub async fn upload_file(
    app: AppHandle,
    state: State<'_, AppState>,
    bucket: String,
    key: String,
    file_path: String,
) -> AppResult<()> {
    let client = get_client(&state).await?;

    let path = PathBuf::from(&file_path);
    if !path.exists() {
        return Err(AppError::Validation(format!(
            "File not found: {}",
            file_path
        )));
    }

    let file_size = tokio::fs::metadata(&path)
        .await?
        .len();

    let content_type = mime_guess::from_path(&path)
        .first_or_octet_stream()
        .to_string();

    let transfer_id = uuid::Uuid::new_v4().to_string();

    // Emit initial progress
    let _ = app.emit("transfer-progress", TransferProgress {
        id: transfer_id.clone(),
        file_name: path.file_name().unwrap_or_default().to_string_lossy().to_string(),
        bytes_transferred: 0,
        total_bytes: file_size,
        status: TransferStatus::InProgress,
    });

    // For files > 100MB, use multipart upload
    const MULTIPART_THRESHOLD: u64 = 100 * 1024 * 1024;
    const PART_SIZE: u64 = 10 * 1024 * 1024; // 10MB parts

    if file_size > MULTIPART_THRESHOLD {
        upload_multipart(&client, &app, &transfer_id, &bucket, &key, &path, file_size, &content_type, PART_SIZE).await?;
    } else {
        let body = aws_sdk_s3::primitives::ByteStream::from_path(&path)
            .await
            .map_err(|e| AppError::S3(e.to_string()))?;

        client
            .put_object()
            .bucket(&bucket)
            .key(&key)
            .content_type(&content_type)
            .body(body)
            .send()
            .await
            .map_err(|e| AppError::S3(e.to_string()))?;
    }

    // Emit completion
    let _ = app.emit("transfer-progress", TransferProgress {
        id: transfer_id,
        file_name: path.file_name().unwrap_or_default().to_string_lossy().to_string(),
        bytes_transferred: file_size,
        total_bytes: file_size,
        status: TransferStatus::Completed,
    });

    Ok(())
}

async fn upload_multipart(
    client: &aws_sdk_s3::Client,
    app: &AppHandle,
    transfer_id: &str,
    bucket: &str,
    key: &str,
    path: &PathBuf,
    file_size: u64,
    content_type: &str,
    part_size: u64,
) -> AppResult<()> {
    let create_resp = client
        .create_multipart_upload()
        .bucket(bucket)
        .key(key)
        .content_type(content_type)
        .send()
        .await
        .map_err(|e| AppError::S3(e.to_string()))?;

    let upload_id = create_resp
        .upload_id()
        .ok_or_else(|| AppError::S3("No upload ID returned".into()))?
        .to_string();

    let mut completed_parts = Vec::new();
    let mut bytes_sent: u64 = 0;
    let mut part_number: i32 = 1;

    let file = tokio::fs::read(&path).await?;

    let result = async {
        for chunk_start in (0..file_size).step_by(part_size as usize) {
            let chunk_end = std::cmp::min(chunk_start + part_size, file_size) as usize;
            let chunk = &file[chunk_start as usize..chunk_end];

            let stream = aws_sdk_s3::primitives::ByteStream::from(chunk.to_vec());

            let upload_resp = client
                .upload_part()
                .bucket(bucket)
                .key(key)
                .upload_id(&upload_id)
                .part_number(part_number)
                .body(stream)
                .send()
                .await
                .map_err(|e| AppError::S3(e.to_string()))?;

            let completed = aws_sdk_s3::types::CompletedPart::builder()
                .e_tag(upload_resp.e_tag().unwrap_or_default())
                .part_number(part_number)
                .build();

            completed_parts.push(completed);
            bytes_sent += chunk.len() as u64;
            part_number += 1;

            let _ = app.emit("transfer-progress", TransferProgress {
                id: transfer_id.to_string(),
                file_name: path.file_name().unwrap_or_default().to_string_lossy().to_string(),
                bytes_transferred: bytes_sent,
                total_bytes: file_size,
                status: TransferStatus::InProgress,
            });
        }

        Ok::<(), AppError>(())
    }.await;

    if let Err(e) = result {
        // Abort the multipart upload on failure
        let _ = client
            .abort_multipart_upload()
            .bucket(bucket)
            .key(key)
            .upload_id(&upload_id)
            .send()
            .await;
        return Err(e);
    }

    let completed_upload = aws_sdk_s3::types::CompletedMultipartUpload::builder()
        .set_parts(Some(completed_parts))
        .build();

    client
        .complete_multipart_upload()
        .bucket(bucket)
        .key(key)
        .upload_id(&upload_id)
        .multipart_upload(completed_upload)
        .send()
        .await
        .map_err(|e| AppError::S3(e.to_string()))?;

    Ok(())
}

#[tauri::command]
pub async fn download_file(
    app: AppHandle,
    state: State<'_, AppState>,
    bucket: String,
    key: String,
    dest_path: String,
) -> AppResult<()> {
    let client = get_client(&state).await?;

    // Sanitize the destination filename
    let dest = PathBuf::from(&dest_path);
    if let Some(file_name) = dest.file_name() {
        let sanitized = sanitize_filename::sanitize(file_name.to_string_lossy());
        if sanitized.is_empty() {
            return Err(AppError::Validation("Invalid destination filename".into()));
        }
    }

    let transfer_id = uuid::Uuid::new_v4().to_string();
    let file_name = key.split('/').last().unwrap_or(&key).to_string();

    // Get object size first
    let head = client
        .head_object()
        .bucket(&bucket)
        .key(&key)
        .send()
        .await
        .map_err(|e| AppError::S3(e.to_string()))?;

    let total_bytes = head.content_length().unwrap_or(0) as u64;

    let _ = app.emit("transfer-progress", TransferProgress {
        id: transfer_id.clone(),
        file_name: file_name.clone(),
        bytes_transferred: 0,
        total_bytes,
        status: TransferStatus::InProgress,
    });

    let resp = client
        .get_object()
        .bucket(&bucket)
        .key(&key)
        .send()
        .await
        .map_err(|e| AppError::S3(e.to_string()))?;

    // Stream to file
    let mut file = tokio::fs::File::create(&dest).await?;
    let mut stream = resp.body.into_async_read();
    let mut bytes_written: u64 = 0;
    let mut buf = vec![0u8; 8 * 1024 * 1024]; // 8MB buffer

    loop {
        let n = tokio::io::AsyncReadExt::read(&mut stream, &mut buf).await?;
        if n == 0 {
            break;
        }
        file.write_all(&buf[..n]).await?;
        bytes_written += n as u64;

        let _ = app.emit("transfer-progress", TransferProgress {
            id: transfer_id.clone(),
            file_name: file_name.clone(),
            bytes_transferred: bytes_written,
            total_bytes,
            status: TransferStatus::InProgress,
        });
    }

    file.flush().await?;

    let _ = app.emit("transfer-progress", TransferProgress {
        id: transfer_id,
        file_name,
        bytes_transferred: total_bytes,
        total_bytes,
        status: TransferStatus::Completed,
    });

    Ok(())
}

/// Get object content as base64 for preview (images, text, etc.)
/// Limited to 50MB to prevent memory issues
#[tauri::command]
pub async fn get_object_preview(
    state: State<'_, AppState>,
    bucket: String,
    key: String,
    max_bytes: Option<u64>,
) -> AppResult<PreviewData> {
    let client = get_client(&state).await?;

    let max = max_bytes.unwrap_or(50 * 1024 * 1024); // 50MB default limit

    // Check size first
    let head = client
        .head_object()
        .bucket(&bucket)
        .key(&key)
        .send()
        .await
        .map_err(|e| AppError::S3(e.to_string()))?;

    let size = head.content_length().unwrap_or(0) as u64;
    let content_type = head
        .content_type()
        .unwrap_or("application/octet-stream")
        .to_string();

    if size > max {
        return Ok(PreviewData {
            content_type,
            size,
            data: None,
            too_large: true,
        });
    }

    let resp = client
        .get_object()
        .bucket(&bucket)
        .key(&key)
        .send()
        .await
        .map_err(|e| AppError::S3(e.to_string()))?;

    let bytes = resp
        .body
        .collect()
        .await
        .map_err(|e| AppError::S3(e.to_string()))?
        .into_bytes();

    use base64::Engine;
    let encoded = base64::engine::general_purpose::STANDARD.encode(&bytes);

    Ok(PreviewData {
        content_type,
        size,
        data: Some(encoded),
        too_large: false,
    })
}

#[derive(serde::Serialize)]
pub struct PreviewData {
    pub content_type: String,
    pub size: u64,
    pub data: Option<String>,
    pub too_large: bool,
}
