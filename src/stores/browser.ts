import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { useS3, type BucketInfo } from '@/composables/useS3'

export interface BrowserItem {
  key: string
  name: string
  size: number
  lastModified: string | null
  storageClass: string | null
  isFolder: boolean
}

export const useBrowserStore = defineStore('browser', () => {
  const s3 = useS3()

  const buckets = ref<BucketInfo[]>([])
  const currentBucket = ref<string | null>(null)
  const currentPrefix = ref('')
  const items = ref<BrowserItem[]>([])
  const loading = ref(false)
  const error = ref('')

  const breadcrumbs = computed(() => {
    if (!currentPrefix.value) return []
    const parts = currentPrefix.value.split('/').filter(Boolean)
    return parts.map((part, i) => ({
      name: part,
      prefix: parts.slice(0, i + 1).join('/') + '/',
    }))
  })

  async function loadBuckets() {
    loading.value = true
    error.value = ''
    try {
      buckets.value = await s3.listBuckets()
    } catch (e: any) {
      error.value = typeof e === 'string' ? e : e.message ?? 'Error'
    } finally {
      loading.value = false
    }
  }

  async function navigateToBucket(bucket: string) {
    currentBucket.value = bucket
    currentPrefix.value = ''
    await loadObjects()
  }

  async function navigateToPrefix(prefix: string) {
    currentPrefix.value = prefix
    await loadObjects()
  }

  async function navigateUp() {
    if (!currentPrefix.value) {
      currentBucket.value = null
      items.value = []
      return
    }
    const parts = currentPrefix.value.split('/').filter(Boolean)
    parts.pop()
    currentPrefix.value = parts.length > 0 ? parts.join('/') + '/' : ''
    await loadObjects()
  }

  async function loadObjects() {
    if (!currentBucket.value) return
    loading.value = true
    error.value = ''
    try {
      const result = await s3.listObjects(currentBucket.value, currentPrefix.value)

      const folders: BrowserItem[] = result.common_prefixes.map((prefix) => {
        const name = prefix.slice(currentPrefix.value.length).replace(/\/$/, '')
        return {
          key: prefix,
          name,
          size: 0,
          lastModified: null,
          storageClass: null,
          isFolder: true,
        }
      })

      const files: BrowserItem[] = result.objects.map((obj) => ({
        key: obj.key,
        name: obj.display_name,
        size: obj.size,
        lastModified: obj.last_modified,
        storageClass: obj.storage_class,
        isFolder: false,
      }))

      items.value = [...folders, ...files]
    } catch (e: any) {
      error.value = typeof e === 'string' ? e : e.message ?? 'Error'
    } finally {
      loading.value = false
    }
  }

  async function refresh() {
    await Promise.all([loadBuckets(), currentBucket.value ? loadObjects() : Promise.resolve()])
  }

  async function createBucket(name: string) {
    await s3.createBucket(name)
    await loadBuckets()
  }

  async function removeBucket(name: string) {
    await s3.deleteBucket(name)
    if (currentBucket.value === name) {
      currentBucket.value = null
      items.value = []
    }
    await loadBuckets()
  }

  async function createFolder(name: string) {
    if (!currentBucket.value) return
    const key = currentPrefix.value + name
    await s3.createFolder(currentBucket.value, key)
    await loadObjects()
  }

  async function deleteItem(item: BrowserItem) {
    if (!currentBucket.value) return
    await s3.deleteObject(currentBucket.value, item.key)
    await loadObjects()
  }

  async function deleteItems(itemKeys: string[]) {
    if (!currentBucket.value) return
    await s3.deleteObjects(currentBucket.value, itemKeys)
    await loadObjects()
  }

  async function uploadFiles(filePaths: string[]) {
    if (!currentBucket.value) return
    for (const path of filePaths) {
      const fileName = path.split(/[\\/]/).pop() ?? 'file'
      const key = currentPrefix.value + fileName
      await s3.uploadFile(currentBucket.value, key, path)
    }
    await loadObjects()
  }

  return {
    buckets,
    currentBucket,
    currentPrefix,
    items,
    loading,
    error,
    breadcrumbs,
    loadBuckets,
    navigateToBucket,
    navigateToPrefix,
    navigateUp,
    loadObjects,
    refresh,
    createBucket,
    removeBucket,
    createFolder,
    deleteItem,
    deleteItems,
    uploadFiles,
  }
})
