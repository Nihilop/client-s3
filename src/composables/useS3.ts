import { invoke } from '@tauri-apps/api/core'

export interface BucketInfo {
  name: string
  creation_date: string | null
}

export interface ObjectInfo {
  key: string
  display_name: string
  size: number
  last_modified: string | null
  etag: string | null
  storage_class: string | null
  is_folder: boolean
}

export interface ObjectDetails {
  key: string
  size: number
  last_modified: string | null
  etag: string | null
  content_type: string | null
  storage_class: string | null
  metadata: Record<string, string>
}

export interface ListObjectsResult {
  objects: ObjectInfo[]
  common_prefixes: string[]
  next_continuation_token: string | null
  is_truncated: boolean
}

export interface ObjectVersion {
  version_id: string | null
  is_latest: boolean
  last_modified: string | null
  size: number
  etag: string | null
  is_delete_marker: boolean
}

export interface PreviewData {
  content_type: string
  size: number
  data: string | null
  too_large: boolean
}

export function useS3() {
  async function createBucket(bucket: string) {
    return invoke<void>('create_bucket', { bucket })
  }

  async function deleteBucket(bucket: string) {
    return invoke<void>('delete_bucket', { bucket })
  }

  async function listBuckets() {
    return invoke<BucketInfo[]>('list_buckets')
  }

  async function listObjects(
    bucket: string,
    prefix: string,
    continuationToken?: string | null,
    maxKeys?: number,
  ) {
    return invoke<ListObjectsResult>('list_objects', {
      bucket,
      prefix,
      continuationToken,
      maxKeys,
    })
  }

  async function headObject(bucket: string, key: string) {
    return invoke<ObjectDetails>('head_object', { bucket, key })
  }

  async function deleteObject(bucket: string, key: string) {
    return invoke<void>('delete_object', { bucket, key })
  }

  async function deleteObjects(bucket: string, keys: string[]) {
    return invoke<string[]>('delete_objects', { bucket, keys })
  }

  async function createFolder(bucket: string, prefix: string) {
    return invoke<void>('create_folder', { bucket, prefix })
  }

  async function copyObject(
    sourceBucket: string,
    sourceKey: string,
    destBucket: string,
    destKey: string,
  ) {
    return invoke<void>('copy_object', {
      sourceBucket,
      sourceKey,
      destBucket,
      destKey,
    })
  }

  async function moveObject(
    sourceBucket: string,
    sourceKey: string,
    destBucket: string,
    destKey: string,
  ) {
    return invoke<void>('move_object', {
      sourceBucket,
      sourceKey,
      destBucket,
      destKey,
    })
  }

  async function getPresignedUrl(bucket: string, key: string, expiresInSecs?: number) {
    return invoke<string>('get_presigned_url', { bucket, key, expiresInSecs })
  }

  async function uploadFile(bucket: string, key: string, filePath: string) {
    return invoke<void>('upload_file', { bucket, key, filePath })
  }

  async function downloadFile(bucket: string, key: string, destPath: string) {
    return invoke<void>('download_file', { bucket, key, destPath })
  }

  async function getObjectPreview(bucket: string, key: string, maxBytes?: number) {
    return invoke<PreviewData>('get_object_preview', { bucket, key, maxBytes })
  }

  async function searchObjects(bucket: string, query: string, maxResults?: number) {
    return invoke<ObjectInfo[]>('search_objects', { bucket, query, maxResults })
  }

  async function getPrefixSize(bucket: string, prefix: string) {
    return invoke<{ total_size: number; object_count: number }>('get_prefix_size', { bucket, prefix })
  }

  async function listObjectVersions(bucket: string, key: string) {
    return invoke<ObjectVersion[]>('list_object_versions', { bucket, key })
  }

  async function getBucketVersioning(bucket: string) {
    return invoke<{ enabled: boolean; mfa_delete: boolean }>('get_bucket_versioning', { bucket })
  }

  async function deleteObjectVersion(bucket: string, key: string, versionId: string) {
    return invoke<void>('delete_object_version', { bucket, key, versionId })
  }

  async function restoreObjectVersion(bucket: string, key: string, versionId: string) {
    return invoke<void>('restore_object_version', { bucket, key, versionId })
  }

  return {
    searchObjects,
    getPrefixSize,
    createBucket,
    deleteBucket,
    listBuckets,
    listObjects,
    headObject,
    deleteObject,
    deleteObjects,
    createFolder,
    copyObject,
    moveObject,
    getPresignedUrl,
    uploadFile,
    downloadFile,
    getObjectPreview,
    listObjectVersions,
    getBucketVersioning,
    deleteObjectVersion,
    restoreObjectVersion,
  }
}
