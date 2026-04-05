<script setup lang="ts">
import { ref, nextTick, onMounted, onUnmounted } from 'vue'
import { useBrowserStore, type BrowserItem } from '@/stores/browser'
import { useS3 } from '@/composables/useS3'
import { useI18n } from 'vue-i18n'
import { save } from '@tauri-apps/plugin-dialog'
import { Input } from '@/components/ui/input'
import PreviewSheet from '@/components/preview/PreviewSheet.vue'
import PropertiesDialog from '@/components/dialogs/PropertiesDialog.vue'
import VersionsDialog from '@/components/dialogs/VersionsDialog.vue'
import ShareDialog from '@/components/dialogs/ShareDialog.vue'
import {
  ContextMenu,
  ContextMenuContent,
  ContextMenuItem,
  ContextMenuTrigger,
  ContextMenuSeparator,
} from '@/components/ui/context-menu'
import { ScrollArea } from '@/components/ui/scroll-area'
import { Spinner } from '@/components/ui/spinner'
import {
  Folder,
  File,
  FileText,
  FileImage,
  FileVideo,
  FileAudio,
  FileArchive,
  Download,
  Trash2,
  Copy,
  Link,
  FolderPlus,
  Upload,
  ArrowUp,
  FileCode,
  Eye,
  Clipboard,
  ClipboardPaste,
  Scissors,
  Pencil,
  Info,
  Star,
  Calculator,
  History,
} from 'lucide-vue-next'
import { toast } from 'vue-sonner'
import { useBookmarkStore } from '@/stores/bookmarks'

const { t, locale } = useI18n()

const emit = defineEmits<{
  createFolder: []
  upload: []
}>()

const store = useBrowserStore()
const s3 = useS3()
const bookmarkStore = useBookmarkStore()
const selectedKeys = ref<Set<string>>(new Set())

// Folder sizes (on demand)
const folderSizes = ref<Map<string, { totalSize: number; objectCount: number } | 'loading'>>(new Map())

async function calculateFolderSize(item: BrowserItem) {
  if (!store.currentBucket) return
  folderSizes.value.set(item.key, 'loading')
  try {
    const result = await s3.getPrefixSize(store.currentBucket, item.key)
    folderSizes.value.set(item.key, { totalSize: result.total_size, objectCount: result.object_count })
    const name = item.name
    toast.success(t('browser.toast.folderSize', {
      name,
      size: formatSize(result.total_size === 0 ? 1 : result.total_size),
      count: result.object_count,
    }))
  } catch (e: any) {
    folderSizes.value.delete(item.key)
    toast.error(typeof e === 'string' ? e : e.message)
  }
}

function getFolderSizeDisplay(key: string): string | null {
  const entry = folderSizes.value.get(key)
  if (!entry || entry === 'loading') return null
  const sizeStr = formatSize(entry.totalSize || 1)
  return `${sizeStr} (${entry.objectCount})`
}

function isFolderSizeLoading(key: string): boolean {
  return folderSizes.value.get(key) === 'loading'
}

// Clipboard for copy/cut/paste
const clipboard = ref<{ bucket: string; keys: string[]; isCut: boolean } | null>(null)

// Track last clicked index for Shift+Click range selection
const lastClickedIndex = ref<number>(-1)

// Rename
const renamingKey = ref<string | null>(null)
const renameValue = ref('')
const renameInputRef = ref<InstanceType<typeof Input> | null>(null)

function startRename(item: BrowserItem) {
  renamingKey.value = item.key
  renameValue.value = item.name
  nextTick(() => {
    const el = renameInputRef.value?.$el as HTMLInputElement | undefined
    if (el) {
      el.focus()
      // Select filename without extension for files
      if (!item.isFolder) {
        const dotIndex = item.name.lastIndexOf('.')
        if (dotIndex > 0) {
          el.setSelectionRange(0, dotIndex)
        } else {
          el.select()
        }
      } else {
        el.select()
      }
    }
  })
}

async function confirmRename(item: BrowserItem) {
  if (!store.currentBucket) return
  const trimmed = renameValue.value.trim()
  if (!trimmed || trimmed === item.name) {
    cancelRename()
    return
  }
  const bucket = store.currentBucket
  const oldKey = item.key

  // Compute new key: replace last segment of the key
  let newKey: string
  if (item.isFolder) {
    // Folder keys end with '/', e.g. "prefix/foldername/"
    const withoutTrailing = oldKey.slice(0, -1)
    const lastSlash = withoutTrailing.lastIndexOf('/')
    const prefix = lastSlash >= 0 ? withoutTrailing.slice(0, lastSlash + 1) : ''
    newKey = prefix + trimmed + '/'
  } else {
    const lastSlash = oldKey.lastIndexOf('/')
    const prefix = lastSlash >= 0 ? oldKey.slice(0, lastSlash + 1) : ''
    newKey = prefix + trimmed
  }

  try {
    await s3.moveObject(bucket, oldKey, bucket, newKey)
    toast.success(t('browser.toast.renamed', { old: item.name, new: trimmed }))
    renamingKey.value = null
    await store.loadObjects()
  } catch (e: any) {
    toast.error(typeof e === 'string' ? e : e.message)
  }
}

function cancelRename() {
  renamingKey.value = null
}

// Share dialog
const shareOpen = ref(false)
const shareBucket = ref('')
const shareKey = ref('')

function openShare(item: BrowserItem) {
  if (item.isFolder || !store.currentBucket) return
  shareBucket.value = store.currentBucket
  shareKey.value = item.key
  shareOpen.value = true
}

// Versions dialog
const versionsOpen = ref(false)
const versionsKey = ref('')

function openVersions(item: BrowserItem) {
  if (item.isFolder) return
  versionsKey.value = item.key
  versionsOpen.value = true
}

// Properties dialog
const propertiesOpen = ref(false)
const propertiesKey = ref('')

function openProperties(item: BrowserItem) {
  if (item.isFolder) return
  propertiesKey.value = item.key
  propertiesOpen.value = true
}

// Preview
const previewOpen = ref(false)
const previewKey = ref('')

function openPreview(item: BrowserItem) {
  if (item.isFolder) return
  previewKey.value = item.key
  previewOpen.value = true
}

function getFileIcon(name: string) {
  const ext = name.split('.').pop()?.toLowerCase() ?? ''
  const imageExts = ['png', 'jpg', 'jpeg', 'gif', 'webp', 'svg', 'bmp', 'ico']
  const videoExts = ['mp4', 'webm', 'avi', 'mkv', 'mov']
  const audioExts = ['mp3', 'wav', 'ogg', 'flac', 'aac']
  const archiveExts = ['zip', 'tar', 'gz', 'rar', '7z', 'bz2']
  const codeExts = ['js', 'ts', 'vue', 'jsx', 'tsx', 'py', 'rs', 'go', 'java', 'css', 'html', 'json', 'yaml', 'toml']
  const textExts = ['txt', 'md', 'csv', 'log', 'xml']

  if (imageExts.includes(ext)) return FileImage
  if (videoExts.includes(ext)) return FileVideo
  if (audioExts.includes(ext)) return FileAudio
  if (archiveExts.includes(ext)) return FileArchive
  if (codeExts.includes(ext)) return FileCode
  if (textExts.includes(ext)) return FileText
  return File
}

function formatSize(bytes: number): string {
  if (bytes === 0) return '-'
  const units = locale.value === 'fr' ? ['o', 'Ko', 'Mo', 'Go', 'To'] : ['B', 'KB', 'MB', 'GB', 'TB']
  const i = Math.floor(Math.log(bytes) / Math.log(1024))
  return `${(bytes / Math.pow(1024, i)).toFixed(i > 0 ? 1 : 0)} ${units[i]}`
}

function formatDate(date: string | null): string {
  if (!date) return '-'
  try {
    return new Date(date).toLocaleDateString(locale.value === 'fr' ? 'fr-FR' : 'en-US', {
      day: '2-digit',
      month: '2-digit',
      year: 'numeric',
      hour: '2-digit',
      minute: '2-digit',
    })
  } catch {
    return date
  }
}

function formatStorageClass(sc: string | null): string {
  if (!sc) return '-'
  const map: Record<string, string> = {
    STANDARD: 'Std',
    INTELLIGENT_TIERING: 'IT',
    STANDARD_IA: 'Std-IA',
    ONEZONE_IA: 'OZ-IA',
    GLACIER: 'Glacier',
    GLACIER_IR: 'Glacier-IR',
    DEEP_ARCHIVE: 'Deep',
    REDUCED_REDUNDANCY: 'RR',
    EXPRESS_ONEZONE: 'Express',
  }
  return map[sc] ?? sc
}

// Bookmark current location
function bookmarkCurrentLocation() {
  if (!store.currentBucket) return
  const prefix = store.currentPrefix
  let defaultName: string
  if (prefix) {
    const parts = prefix.split('/').filter(Boolean)
    defaultName = parts[parts.length - 1] ?? store.currentBucket
  } else {
    defaultName = store.currentBucket
  }
  const name = window.prompt(t('bookmarks.namePrompt'), defaultName)
  if (!name) return
  bookmarkStore.addBookmark(name, store.currentBucket, prefix)
  toast.success(t('bookmarks.added'))
}

function handleClick(item: BrowserItem, event: MouseEvent) {
  const currentIndex = store.items.findIndex(i => i.key === item.key)

  // Shift+Click = range selection from last clicked to current
  if (event.shiftKey && lastClickedIndex.value >= 0) {
    const start = Math.min(lastClickedIndex.value, currentIndex)
    const end = Math.max(lastClickedIndex.value, currentIndex)
    // If not holding Ctrl, clear first
    if (!event.ctrlKey && !event.metaKey) {
      selectedKeys.value.clear()
    }
    for (let i = start; i <= end; i++) {
      selectedKeys.value.add(store.items[i].key)
    }
    // Don't update lastClickedIndex on shift-click to allow extending range
    return
  }

  // Ctrl+Click = toggle selection (works for files AND folders)
  if (event.ctrlKey || event.metaKey) {
    if (selectedKeys.value.has(item.key)) {
      selectedKeys.value.delete(item.key)
    } else {
      selectedKeys.value.add(item.key)
    }
    lastClickedIndex.value = currentIndex
    return
  }

  // Normal click on folder = navigate
  if (item.isFolder) {
    store.navigateToPrefix(item.key)
    return
  }

  // Normal click on file = select only this one
  selectedKeys.value.clear()
  selectedKeys.value.add(item.key)
  lastClickedIndex.value = currentIndex
}

function isSelected(item: BrowserItem) {
  return selectedKeys.value.has(item.key)
}

async function downloadItem(item: BrowserItem) {
  if (!store.currentBucket) return
  const destPath = await save({ defaultPath: item.name })
  if (!destPath) return
  try {
    await s3.downloadFile(store.currentBucket, item.key, destPath)
    toast.success(t('browser.toast.downloaded', { name: item.name }))
  } catch (e: any) {
    toast.error(typeof e === 'string' ? e : e.message)
  }
}

async function deleteItem(item: BrowserItem) {
  try {
    await store.deleteItem(item)
    selectedKeys.value.delete(item.key)
    toast.success(t('browser.toast.deleted', { name: item.name }))
  } catch (e: any) {
    toast.error(typeof e === 'string' ? e : e.message)
  }
}

async function copyUrl(item: BrowserItem) {
  if (!store.currentBucket) return
  try {
    const url = await s3.getPresignedUrl(store.currentBucket, item.key)
    await navigator.clipboard.writeText(url)
    toast.success(t('browser.toast.urlCopied'))
  } catch (e: any) {
    toast.error(typeof e === 'string' ? e : e.message)
  }
}

async function deleteSelected() {
  if (selectedKeys.value.size === 0) return
  try {
    await store.deleteItems([...selectedKeys.value])
    const count = selectedKeys.value.size
    selectedKeys.value.clear()
    toast.success(t('browser.toast.selectionDeleted', { count }))
  } catch (e: any) {
    toast.error(typeof e === 'string' ? e : e.message)
  }
}

function copySelection() {
  if (selectedKeys.value.size === 0 || !store.currentBucket) return
  clipboard.value = {
    bucket: store.currentBucket,
    keys: [...selectedKeys.value],
    isCut: false,
  }
  toast.success(t('browser.toast.copied', { count: selectedKeys.value.size }))
}

function cutSelection() {
  if (selectedKeys.value.size === 0 || !store.currentBucket) return
  clipboard.value = {
    bucket: store.currentBucket,
    keys: [...selectedKeys.value],
    isCut: true,
  }
  toast.success(t('browser.toast.cut', { count: selectedKeys.value.size }))
}

async function pasteClipboard() {
  if (!clipboard.value || !store.currentBucket) return
  const { bucket: srcBucket, keys, isCut } = clipboard.value
  try {
    for (const sourceKey of keys) {
      const fileName = sourceKey.split('/').filter(Boolean).pop() ?? sourceKey
      const destKey = store.currentPrefix + fileName
      if (isCut) {
        await s3.moveObject(srcBucket, sourceKey, store.currentBucket, destKey)
      } else {
        await s3.copyObject(srcBucket, sourceKey, store.currentBucket, destKey)
      }
    }
    toast.success(t(isCut ? 'browser.toast.moved' : 'browser.toast.pasted', { count: keys.length }))
    if (isCut) clipboard.value = null
    await store.loadObjects()
  } catch (e: any) {
    toast.error(typeof e === 'string' ? e : e.message)
  }
}

// Keyboard shortcuts
function handleKeydown(e: KeyboardEvent) {
  // Ignore if user is typing in an input
  const tag = (e.target as HTMLElement)?.tagName
  if (tag === 'INPUT' || tag === 'TEXTAREA') return

  // Del = delete selection
  if (e.key === 'Delete' && selectedKeys.value.size > 0) {
    e.preventDefault()
    deleteSelected()
    return
  }

  // Ctrl+C = copy selection
  if ((e.ctrlKey || e.metaKey) && e.key === 'c' && selectedKeys.value.size > 0) {
    e.preventDefault()
    copySelection()
    return
  }

  // Ctrl+X = cut selection
  if ((e.ctrlKey || e.metaKey) && e.key === 'x' && selectedKeys.value.size > 0) {
    e.preventDefault()
    cutSelection()
    return
  }

  // Ctrl+V = paste
  if ((e.ctrlKey || e.metaKey) && e.key === 'v' && clipboard.value) {
    e.preventDefault()
    pasteClipboard()
    return
  }

  // Ctrl+A = select all
  if ((e.ctrlKey || e.metaKey) && e.key === 'a' && store.items.length > 0) {
    e.preventDefault()
    selectedKeys.value.clear()
    for (const item of store.items) {
      selectedKeys.value.add(item.key)
    }
    return
  }

  // F2 = rename first selected item
  if (e.key === 'F2' && selectedKeys.value.size > 0) {
    e.preventDefault()
    const firstKey = [...selectedKeys.value][0]
    const item = store.items.find(i => i.key === firstKey)
    if (item) startRename(item)
    return
  }

  // Alt+Enter = open properties for first selected file
  if (e.altKey && e.key === 'Enter' && selectedKeys.value.size > 0) {
    e.preventDefault()
    const firstKey = [...selectedKeys.value][0]
    const item = store.items.find(i => i.key === firstKey)
    if (item && !item.isFolder) openProperties(item)
    return
  }

  // Escape = clear selection
  if (e.key === 'Escape') {
    selectedKeys.value.clear()
  }
}

onMounted(() => {
  window.addEventListener('keydown', handleKeydown)
})
onUnmounted(() => {
  window.removeEventListener('keydown', handleKeydown)
})
</script>

<template>
  <!-- Whole area context menu -->
  <ContextMenu>
    <ContextMenuTrigger as-child>
      <ScrollArea class="h-full">
        <div v-if="store.loading" class="flex h-64 items-center justify-center">
          <Spinner class="size-6 text-muted-foreground" />
        </div>

        <div v-else-if="!store.currentBucket" class="flex h-64 items-center justify-center text-sm text-muted-foreground">
          {{ t('browser.selectBucket') }}
        </div>

        <div v-else class="min-h-full">
          <!-- Table header -->
          <div class="sticky top-0 z-10 grid grid-cols-[1fr_100px_80px_160px] gap-2 border-b border-border bg-muted/50 px-3 py-1.5 text-xs font-medium text-muted-foreground">
            <span>{{ t('browser.table.name') }}</span>
            <span class="text-right">{{ t('browser.table.size') }}</span>
            <span class="text-right">{{ t('browser.table.class') }}</span>
            <span class="text-right">{{ t('browser.table.modified') }}</span>
          </div>

          <!-- Go up -->
          <div
            v-if="store.currentPrefix"
            class="grid cursor-pointer select-none grid-cols-[1fr_100px_80px_160px] gap-2 border-b border-border/50 px-3 py-1.5 text-sm transition-colors hover:bg-accent"
            @click="store.navigateUp()"
          >
            <div class="flex items-center gap-2">
              <ArrowUp class="size-4 text-muted-foreground" />
              <span>..</span>
            </div>
            <span />
            <span />
            <span />
          </div>

          <!-- Items -->
          <template v-for="item in store.items" :key="item.key">
            <ContextMenu>
              <ContextMenuTrigger as-child>
                <div
                  class="grid cursor-pointer select-none grid-cols-[1fr_100px_80px_160px] gap-2 border-b border-border/50 px-3 py-1.5 text-sm transition-colors hover:bg-accent"
                  :class="{ 'bg-accent': isSelected(item) }"
                  @click="handleClick(item, $event)"
                  @dblclick="item.isFolder ? store.navigateToPrefix(item.key) : openPreview(item)"
                >
                  <div class="flex items-center gap-2 overflow-hidden">
                    <Folder v-if="item.isFolder" class="size-4 shrink-0 text-blue-500" />
                    <component v-else :is="getFileIcon(item.name)" class="size-4 shrink-0 text-muted-foreground" />
                    <Input
                      v-if="renamingKey === item.key"
                      ref="renameInputRef"
                      v-model="renameValue"
                      class="h-6 text-sm px-1 py-0"
                      @keydown.enter="confirmRename(item)"
                      @keydown.escape="cancelRename()"
                      @blur="confirmRename(item)"
                      @click.stop
                    />
                    <span v-else class="truncate select-none">{{ item.name }}</span>
                  </div>
                  <span class="text-right text-muted-foreground">
                    <template v-if="item.isFolder && isFolderSizeLoading(item.key)">
                      <Spinner class="ml-auto size-3" />
                    </template>
                    <template v-else-if="item.isFolder && getFolderSizeDisplay(item.key)">
                      {{ getFolderSizeDisplay(item.key) }}
                    </template>
                    <template v-else>
                      {{ formatSize(item.size) }}
                    </template>
                  </span>
                  <span class="text-right text-muted-foreground">{{ formatStorageClass(item.storageClass) }}</span>
                  <span class="text-right text-muted-foreground">{{ formatDate(item.lastModified) }}</span>
                </div>
              </ContextMenuTrigger>

              <!-- Item context menu -->
              <ContextMenuContent>
                <ContextMenuItem v-if="item.isFolder" @click="store.navigateToPrefix(item.key)">
                  <Folder class="mr-2 size-4" />
                  {{ t('common.open') }}
                </ContextMenuItem>
                <ContextMenuItem v-if="item.isFolder" @click="calculateFolderSize(item)">
                  <Calculator class="mr-2 size-4" />
                  {{ t('browser.context.calculateSize') }}
                </ContextMenuItem>
                <ContextMenuItem v-if="!item.isFolder" @click="openPreview(item)">
                  <Eye class="mr-2 size-4" />
                  {{ t('common.open') }}
                </ContextMenuItem>
                <ContextMenuItem v-if="!item.isFolder" @click="downloadItem(item)">
                  <Download class="mr-2 size-4" />
                  {{ t('common.download') }}
                </ContextMenuItem>
                <ContextMenuItem v-if="!item.isFolder" @click="copyUrl(item)">
                  <Link class="mr-2 size-4" />
                  {{ t('common.copyUrl') }}
                </ContextMenuItem>
                <ContextMenuItem v-if="!item.isFolder" @click="openShare(item)">
                  <Link class="mr-2 size-4" />
                  {{ t('share.title') }}
                </ContextMenuItem>
                <ContextMenuItem v-if="!item.isFolder" @click="openProperties(item)">
                  <Info class="mr-2 size-4" />
                  {{ t('properties.title') }}
                  <span class="ml-auto text-xs text-muted-foreground">Alt+Enter</span>
                </ContextMenuItem>
                <ContextMenuItem v-if="!item.isFolder" @click="openVersions(item)">
                  <History class="mr-2 size-4" />
                  {{ t('versions.title') }}
                </ContextMenuItem>
                <ContextMenuItem @click="startRename(item)">
                  <Pencil class="mr-2 size-4" />
                  {{ t('common.rename') }}
                  <span class="ml-auto text-xs text-muted-foreground">F2</span>
                </ContextMenuItem>
                <ContextMenuItem @click="copySelection()">
                  <Clipboard class="mr-2 size-4" />
                  {{ t('browser.context.copy') }}
                  <span class="ml-auto text-xs text-muted-foreground">Ctrl+C</span>
                </ContextMenuItem>
                <ContextMenuItem @click="cutSelection()">
                  <Scissors class="mr-2 size-4" />
                  {{ t('browser.context.cut') }}
                  <span class="ml-auto text-xs text-muted-foreground">Ctrl+X</span>
                </ContextMenuItem>
                <ContextMenuSeparator />
                <ContextMenuItem class="text-destructive" @click="deleteItem(item)">
                  <Trash2 class="mr-2 size-4" />
                  {{ t('common.delete') }}
                </ContextMenuItem>
                <ContextMenuItem
                  v-if="selectedKeys.size > 1"
                  class="text-destructive"
                  @click="deleteSelected()"
                >
                  <Trash2 class="mr-2 size-4" />
                  {{ t('browser.context.deleteSelection', { count: selectedKeys.size }) }}
                  <span class="ml-auto text-xs text-muted-foreground">Del</span>
                </ContextMenuItem>
              </ContextMenuContent>
            </ContextMenu>
          </template>

          <!-- Empty state -->
          <div
            v-if="store.items.length === 0 && !store.loading"
            class="flex h-48 flex-col items-center justify-center gap-2 text-sm text-muted-foreground"
          >
            <Folder class="size-8 opacity-30" />
            <span>{{ t('browser.emptyFolder') }}</span>
          </div>
        </div>
      </ScrollArea>
    </ContextMenuTrigger>

    <!-- Background context menu -->
    <ContextMenuContent>
      <ContextMenuItem @click="emit('createFolder')">
        <FolderPlus class="mr-2 size-4" />
        {{ t('browser.context.newFolder') }}
      </ContextMenuItem>
      <ContextMenuItem @click="emit('upload')">
        <Upload class="mr-2 size-4" />
        {{ t('browser.context.uploadFiles') }}
      </ContextMenuItem>
      <ContextMenuItem v-if="clipboard" @click="pasteClipboard()">
        <ClipboardPaste class="mr-2 size-4" />
        {{ t('browser.context.paste', { count: clipboard.keys.length }) }}
        <span class="ml-auto text-xs text-muted-foreground">Ctrl+V</span>
      </ContextMenuItem>
      <ContextMenuItem v-if="store.currentBucket" @click="bookmarkCurrentLocation()">
        <Star class="mr-2 size-4" />
        {{ t('bookmarks.add') }}
      </ContextMenuItem>
      <ContextMenuSeparator />
      <ContextMenuItem @click="store.refresh()">
        <Copy class="mr-2 size-4" />
        {{ t('common.refresh') }}
      </ContextMenuItem>
      <ContextMenuItem
        v-if="selectedKeys.size > 0"
        class="text-destructive"
        @click="deleteSelected()"
      >
        <Trash2 class="mr-2 size-4" />
        {{ t('browser.context.deleteSelection', { count: selectedKeys.size }) }}
        <span class="ml-auto text-xs text-muted-foreground">Del</span>
      </ContextMenuItem>
    </ContextMenuContent>
  </ContextMenu>

  <!-- Preview sheet -->
  <PreviewSheet
    v-if="store.currentBucket"
    v-model:open="previewOpen"
    :bucket="store.currentBucket"
    :object-key="previewKey"
  />

  <!-- Properties dialog -->
  <PropertiesDialog
    v-if="store.currentBucket"
    v-model:open="propertiesOpen"
    :bucket="store.currentBucket"
    :object-key="propertiesKey"
  />

  <!-- Versions dialog -->
  <VersionsDialog
    v-if="store.currentBucket"
    v-model:open="versionsOpen"
    :bucket="store.currentBucket"
    :object-key="versionsKey"
  />

  <!-- Share dialog -->
  <ShareDialog
    v-if="shareBucket"
    v-model:open="shareOpen"
    :bucket="shareBucket"
    :object-key="shareKey"
  />
</template>
