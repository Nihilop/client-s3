pub mod commands;
pub mod types;

use aws_config::Region;
use aws_credential_types::Credentials;
use aws_sdk_s3::config::Builder as S3ConfigBuilder;
use aws_sdk_s3::Client as S3Client;

use crate::error::{AppError, AppResult};
use crate::s3::types::*;

/// Create an S3 client from a connection profile
pub async fn create_client(profile: &ConnectionProfile) -> S3Client {
    let credentials = Credentials::new(
        &profile.access_key_id,
        &profile.secret_access_key,
        None,
        None,
        "client-s3",
    );

    let config = S3ConfigBuilder::new()
        .region(Region::new(profile.region.clone()))
        .credentials_provider(credentials)
        .endpoint_url(&profile.endpoint)
        .force_path_style(profile.path_style)
        .build();
    S3Client::from_conf(config)
}

/// Create a new bucket
pub async fn create_bucket(client: &S3Client, bucket: &str) -> AppResult<()> {
    validate_bucket_name(bucket)?;

    client
        .create_bucket()
        .bucket(bucket)
        .send()
        .await
        .map_err(|e| AppError::S3(e.to_string()))?;

    Ok(())
}

/// Delete a bucket (must be empty)
pub async fn delete_bucket(client: &S3Client, bucket: &str) -> AppResult<()> {
    validate_bucket_name(bucket)?;

    client
        .delete_bucket()
        .bucket(bucket)
        .send()
        .await
        .map_err(|e| AppError::S3(e.to_string()))?;

    Ok(())
}

/// List all buckets
pub async fn list_buckets(client: &S3Client) -> AppResult<Vec<BucketInfo>> {
    let resp = client
        .list_buckets()
        .send()
        .await
        .map_err(|e| AppError::S3(e.to_string()))?;

    let buckets = resp
        .buckets()
        .iter()
        .map(|b| BucketInfo {
            name: b.name().unwrap_or_default().to_string(),
            creation_date: b.creation_date().map(|d| d.to_string()),
        })
        .collect();

    Ok(buckets)
}

/// List objects in a bucket with prefix-based "folder" navigation
pub async fn list_objects(
    client: &S3Client,
    bucket: &str,
    prefix: &str,
    continuation_token: Option<&str>,
    max_keys: i32,
) -> AppResult<ListObjectsResult> {
    validate_bucket_name(bucket)?;

    let mut req = client
        .list_objects_v2()
        .bucket(bucket)
        .prefix(prefix)
        .delimiter("/")
        .max_keys(max_keys);

    if let Some(token) = continuation_token {
        req = req.continuation_token(token);
    }

    let resp = req
        .send()
        .await
        .map_err(|e| AppError::S3(e.to_string()))?;

    let objects: Vec<ObjectInfo> = resp
        .contents()
        .iter()
        .filter(|obj| {
            // Skip the prefix itself (the "folder" marker)
            obj.key().unwrap_or_default() != prefix
        })
        .map(|obj| {
            let key = obj.key().unwrap_or_default().to_string();
            let display_name = key
                .strip_prefix(prefix)
                .unwrap_or(&key)
                .to_string();
            ObjectInfo {
                key,
                display_name,
                size: obj.size().unwrap_or(0),
                last_modified: obj.last_modified().map(|d| d.to_string()),
                etag: obj.e_tag().map(|s| s.to_string()),
                storage_class: obj.storage_class().map(|s| s.as_str().to_string()),
                is_folder: false,
            }
        })
        .collect();

    let common_prefixes: Vec<String> = resp
        .common_prefixes()
        .iter()
        .filter_map(|cp| cp.prefix().map(|s| s.to_string()))
        .collect();

    Ok(ListObjectsResult {
        objects,
        common_prefixes,
        next_continuation_token: resp.next_continuation_token().map(|s| s.to_string()),
        is_truncated: resp.is_truncated().unwrap_or(false),
    })
}

/// Get detailed object metadata
pub async fn head_object(
    client: &S3Client,
    bucket: &str,
    key: &str,
) -> AppResult<ObjectDetails> {
    validate_bucket_name(bucket)?;

    let resp = client
        .head_object()
        .bucket(bucket)
        .key(key)
        .send()
        .await
        .map_err(|e| AppError::S3(e.to_string()))?;

    let metadata = resp
        .metadata()
        .map(|m| m.clone())
        .unwrap_or_default();

    Ok(ObjectDetails {
        key: key.to_string(),
        size: resp.content_length().unwrap_or(0),
        last_modified: resp.last_modified().map(|d| d.to_string()),
        etag: resp.e_tag().map(|s| s.to_string()),
        content_type: resp.content_type().map(|s| s.to_string()),
        storage_class: resp.storage_class().map(|s| s.as_str().to_string()),
        metadata,
    })
}

/// Delete a single object
pub async fn delete_object(
    client: &S3Client,
    bucket: &str,
    key: &str,
) -> AppResult<()> {
    validate_bucket_name(bucket)?;

    client
        .delete_object()
        .bucket(bucket)
        .key(key)
        .send()
        .await
        .map_err(|e| AppError::S3(e.to_string()))?;

    Ok(())
}

/// Delete multiple objects
pub async fn delete_objects(
    client: &S3Client,
    bucket: &str,
    keys: &[String],
) -> AppResult<Vec<String>> {
    validate_bucket_name(bucket)?;

    if keys.is_empty() {
        return Ok(Vec::new());
    }

    let objects: Vec<aws_sdk_s3::types::ObjectIdentifier> = keys
        .iter()
        .map(|k| {
            aws_sdk_s3::types::ObjectIdentifier::builder()
                .key(k)
                .build()
                .unwrap()
        })
        .collect();

    let delete = aws_sdk_s3::types::Delete::builder()
        .set_objects(Some(objects))
        .build()
        .map_err(|e| AppError::S3(e.to_string()))?;

    let resp = client
        .delete_objects()
        .bucket(bucket)
        .delete(delete)
        .send()
        .await
        .map_err(|e| AppError::S3(e.to_string()))?;

    let mut errors = Vec::new();
    for err in resp.errors() {
        errors.push(format!(
            "{}: {}",
            err.key().unwrap_or("unknown"),
            err.message().unwrap_or("unknown error")
        ));
    }
    Ok(errors)
}

/// Create a "folder" by putting an empty object with trailing /
pub async fn create_folder(
    client: &S3Client,
    bucket: &str,
    prefix: &str,
) -> AppResult<()> {
    validate_bucket_name(bucket)?;

    let key = if prefix.ends_with('/') {
        prefix.to_string()
    } else {
        format!("{}/", prefix)
    };

    client
        .put_object()
        .bucket(bucket)
        .key(&key)
        .content_length(0)
        .send()
        .await
        .map_err(|e| AppError::S3(e.to_string()))?;

    Ok(())
}

/// Copy an object within or across buckets
pub async fn copy_object(
    client: &S3Client,
    source_bucket: &str,
    source_key: &str,
    dest_bucket: &str,
    dest_key: &str,
) -> AppResult<()> {
    validate_bucket_name(source_bucket)?;
    validate_bucket_name(dest_bucket)?;

    let copy_source = format!("{}/{}", source_bucket, source_key);

    client
        .copy_object()
        .bucket(dest_bucket)
        .key(dest_key)
        .copy_source(&copy_source)
        .send()
        .await
        .map_err(|e| AppError::S3(e.to_string()))?;

    Ok(())
}

/// Generate a presigned URL for object preview/download
pub async fn get_presigned_url(
    client: &S3Client,
    bucket: &str,
    key: &str,
    expires_in_secs: u64,
) -> AppResult<String> {
    validate_bucket_name(bucket)?;

    let presigning_config = aws_sdk_s3::presigning::PresigningConfig::expires_in(
        std::time::Duration::from_secs(expires_in_secs),
    )
    .map_err(|e| AppError::S3(e.to_string()))?;

    let presigned = client
        .get_object()
        .bucket(bucket)
        .key(key)
        .presigned(presigning_config)
        .await
        .map_err(|e| AppError::S3(e.to_string()))?;

    Ok(presigned.uri().to_string())
}

/// Search objects recursively (no delimiter) and filter by substring
pub async fn search_objects(
    client: &S3Client,
    bucket: &str,
    query: &str,
    max_results: usize,
) -> AppResult<Vec<ObjectInfo>> {
    validate_bucket_name(bucket)?;

    let query_lower = query.to_lowercase();
    let mut results = Vec::new();
    let mut continuation_token: Option<String> = None;

    loop {
        let mut req = client
            .list_objects_v2()
            .bucket(bucket)
            .max_keys(1000);

        if let Some(ref token) = continuation_token {
            req = req.continuation_token(token);
        }

        let resp = req
            .send()
            .await
            .map_err(|e| AppError::S3(e.to_string()))?;

        for obj in resp.contents() {
            let key = obj.key().unwrap_or_default().to_string();
            // For folder keys like "foo/bar/", strip trailing slash before extracting name
            let trimmed = key.trim_end_matches('/');
            let name = trimmed.split('/').last().unwrap_or(trimmed).to_string();

            if !name.is_empty() && name.to_lowercase().contains(&query_lower) {
                results.push(ObjectInfo {
                    key: key.clone(),
                    display_name: name,
                    size: obj.size().unwrap_or(0),
                    last_modified: obj.last_modified().map(|d| d.to_string()),
                    etag: obj.e_tag().map(|s| s.to_string()),
                    storage_class: obj.storage_class().map(|s| s.as_str().to_string()),
                    is_folder: key.ends_with('/'),
                });

                if results.len() >= max_results {
                    return Ok(results);
                }
            }
        }

        if resp.is_truncated() != Some(true) {
            break;
        }
        continuation_token = resp.next_continuation_token().map(|s| s.to_string());
    }

    Ok(results)
}

/// Validate bucket name to prevent injection
fn validate_bucket_name(name: &str) -> AppResult<()> {
    if name.is_empty() || name.len() > 63 {
        return Err(AppError::Validation("Invalid bucket name length".into()));
    }
    if !name
        .chars()
        .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-' || c == '.')
    {
        return Err(AppError::Validation(
            "Bucket name contains invalid characters".into(),
        ));
    }
    Ok(())
}
