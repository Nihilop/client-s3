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

  return {
    searchObjects,
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
  }
}
