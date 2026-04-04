<script setup lang="ts">
import { ref } from 'vue'
import { useBrowserStore, type BrowserItem } from '@/stores/browser'
import { useS3 } from '@/composables/useS3'
import { useI18n } from 'vue-i18n'
import { save } from '@tauri-apps/plugin-dialog'
import PreviewSheet from '@/components/preview/PreviewSheet.vue'
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
} from 'lucide-vue-next'
import { toast } from 'vue-sonner'

const { t, locale } = useI18n()

const emit = defineEmits<{
  createFolder: []
  upload: []
}>()

const store = useBrowserStore()
const s3 = useS3()
const selectedKeys = ref<Set<string>>(new Set())

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

function handleClick(item: BrowserItem, event: MouseEvent) {
  if (item.isFolder) {
    store.navigateToPrefix(item.key)
    return
  }

  if (event.ctrlKey || event.metaKey) {
    if (selectedKeys.value.has(item.key)) {
      selectedKeys.value.delete(item.key)
    } else {
      selectedKeys.value.add(item.key)
    }
  } else {
    selectedKeys.value.clear()
    selectedKeys.value.add(item.key)
  }
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
    selectedKeys.value.clear()
    toast.success(t('browser.toast.itemsDeleted'))
  } catch (e: any) {
    toast.error(typeof e === 'string' ? e : e.message)
  }
}
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
          <div class="sticky top-0 z-10 grid grid-cols-[1fr_100px_160px] gap-2 border-b border-border bg-muted/50 px-3 py-1.5 text-xs font-medium text-muted-foreground">
            <span>{{ t('browser.table.name') }}</span>
            <span class="text-right">{{ t('browser.table.size') }}</span>
            <span class="text-right">{{ t('browser.table.modified') }}</span>
          </div>

          <!-- Go up -->
          <div
            v-if="store.currentPrefix"
            class="grid cursor-pointer select-none grid-cols-[1fr_100px_160px] gap-2 border-b border-border/50 px-3 py-1.5 text-sm transition-colors hover:bg-accent"
            @click="store.navigateUp()"
          >
            <div class="flex items-center gap-2">
              <ArrowUp class="size-4 text-muted-foreground" />
              <span>..</span>
            </div>
            <span />
            <span />
          </div>

          <!-- Items -->
          <template v-for="item in store.items" :key="item.key">
            <ContextMenu>
              <ContextMenuTrigger as-child>
                <div
                  class="grid cursor-pointer select-none grid-cols-[1fr_100px_160px] gap-2 border-b border-border/50 px-3 py-1.5 text-sm transition-colors hover:bg-accent"
                  :class="{ 'bg-accent': isSelected(item) }"
                  @click="handleClick(item, $event)"
                  @dblclick="item.isFolder ? store.navigateToPrefix(item.key) : openPreview(item)"
                >
                  <div class="flex items-center gap-2 overflow-hidden">
                    <Folder v-if="item.isFolder" class="size-4 shrink-0 text-blue-500" />
                    <component v-else :is="getFileIcon(item.name)" class="size-4 shrink-0 text-muted-foreground" />
                    <span class="truncate select-none">{{ item.name }}</span>
                  </div>
                  <span class="text-right text-muted-foreground">{{ formatSize(item.size) }}</span>
                  <span class="text-right text-muted-foreground">{{ formatDate(item.lastModified) }}</span>
                </div>
              </ContextMenuTrigger>

              <!-- Item context menu -->
              <ContextMenuContent>
                <ContextMenuItem v-if="item.isFolder" @click="store.navigateToPrefix(item.key)">
                  <Folder class="mr-2 size-4" />
                  {{ t('common.open') }}
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
                <ContextMenuSeparator />
                <ContextMenuItem class="text-destructive" @click="deleteItem(item)">
                  <Trash2 class="mr-2 size-4" />
                  {{ t('common.delete') }}
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
</template>
