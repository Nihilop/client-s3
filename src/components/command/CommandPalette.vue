<script setup lang="ts">
import { ref, watch, onMounted, onUnmounted } from 'vue'
import { useRouter } from 'vue-router'
import { useBrowserStore } from '@/stores/browser'
import { useConnectionStore } from '@/stores/connection'
import { useS3, type ObjectInfo } from '@/composables/useS3'
import { useI18n } from 'vue-i18n'
import {
  CommandDialog,
  CommandEmpty,
  CommandGroup,
  CommandInput,
  CommandItem,
  CommandList,
  CommandSeparator,
} from '@/components/ui/command'
import {
  Database,
  FolderPlus,
  Upload,
  LogOut,
  RefreshCw,
  ArrowUp,
  Plus,
  File,
  Folder,
} from 'lucide-vue-next'

const { t } = useI18n()

const emit = defineEmits<{
  createBucket: []
  createFolder: []
  upload: []
  openFile: [bucket: string, key: string]
}>()

interface SearchResult extends ObjectInfo {
  bucket: string
}

const open = ref(false)
const searchQuery = ref('')
const searchResults = ref<SearchResult[]>([])
const searching = ref(false)
let searchTimeout: ReturnType<typeof setTimeout> | null = null

const router = useRouter()
const browserStore = useBrowserStore()
const connectionStore = useConnectionStore()
const s3 = useS3()

defineExpose({ open })

function handleKeydown(e: KeyboardEvent) {
  if ((e.ctrlKey || e.metaKey) && e.key === 'k') {
    e.preventDefault()
    open.value = !open.value
  }
}

onMounted(() => {
  window.addEventListener('keydown', handleKeydown)
})
onUnmounted(() => {
  window.removeEventListener('keydown', handleKeydown)
})

// Debounced search
watch(searchQuery, (query) => {
  if (searchTimeout) clearTimeout(searchTimeout)
  if (!query || query.length < 2) {
    searchResults.value = []
    searching.value = false
    return
  }
  searching.value = true
  searchTimeout = setTimeout(async () => {
    try {
      const bucketsToSearch = browserStore.currentBucket
        ? [browserStore.currentBucket]
        : browserStore.buckets.map(b => b.name)

      const allResults: SearchResult[] = []
      for (const bucketName of bucketsToSearch) {
        if (allResults.length >= 20) break
        const results = await s3.searchObjects(bucketName, query, 20 - allResults.length)
        allResults.push(...results.map(r => ({ ...r, bucket: bucketName })))
      }
      searchResults.value = allResults
    } catch {
      searchResults.value = []
    } finally {
      searching.value = false
    }
  }, 300)
})

watch(open, (isOpen) => {
  if (!isOpen) {
    searchQuery.value = ''
    searchResults.value = []
  }
})

function runAction(fn: () => void) {
  open.value = false
  fn()
}

async function navigateToFile(result: SearchResult) {
  open.value = false

  // Navigate to the correct bucket first
  if (browserStore.currentBucket !== result.bucket) {
    await browserStore.navigateToBucket(result.bucket)
  }

  if (result.is_folder) {
    browserStore.navigateToPrefix(result.key)
  } else {
    // Navigate to containing folder, then open preview
    const parts = result.key.split('/')
    parts.pop()
    const prefix = parts.length > 0 ? parts.join('/') + '/' : ''
    await browserStore.navigateToPrefix(prefix)
    emit('openFile', result.bucket, result.key)
  }
}
</script>

<template>
  <CommandDialog v-model:open="open">
    <CommandInput v-model="searchQuery" :placeholder="t('command.placeholder')" />
    <CommandList>
      <CommandEmpty>
        {{ searching ? t('command.searching') : t('command.noResults') }}
      </CommandEmpty>

      <!-- File search results -->
      <CommandGroup v-if="searchResults.length > 0" :heading="t('command.groups.files')">
        <CommandItem
          v-for="result in searchResults"
          :key="result.key"
          :value="'file-' + result.key"
          @select="navigateToFile(result)"
        >
          <Folder v-if="result.is_folder" class="mr-2 size-4 text-blue-500" />
          <File v-else class="mr-2 size-4" />
          <div class="flex min-w-0 flex-1 flex-col">
            <span class="truncate text-sm">{{ result.display_name }}</span>
            <span class="truncate text-xs text-muted-foreground">{{ result.bucket }}:/{{ result.key }}</span>
          </div>
        </CommandItem>
      </CommandGroup>

      <CommandSeparator v-if="searchResults.length > 0" />

      <!-- Bucket navigation -->
      <CommandGroup v-if="browserStore.buckets.length > 0" :heading="t('command.groups.buckets')">
        <CommandItem
          v-for="bucket in browserStore.buckets"
          :key="bucket.name"
          :value="'bucket-' + bucket.name"
          @select="runAction(() => browserStore.navigateToBucket(bucket.name))"
        >
          <Database class="mr-2 size-4" />
          {{ bucket.name }}
        </CommandItem>
      </CommandGroup>

      <CommandSeparator />

      <!-- Actions -->
      <CommandGroup :heading="t('command.groups.actions')">
        <CommandItem value="create-bucket" @select="runAction(() => emit('createBucket'))">
          <Plus class="mr-2 size-4" />
          {{ t('command.actions.createBucket') }}
        </CommandItem>
        <CommandItem
          v-if="browserStore.currentBucket"
          value="create-folder"
          @select="runAction(() => emit('createFolder'))"
        >
          <FolderPlus class="mr-2 size-4" />
          {{ t('command.actions.newFolder') }}
        </CommandItem>
        <CommandItem
          v-if="browserStore.currentBucket"
          value="upload"
          @select="runAction(() => emit('upload'))"
        >
          <Upload class="mr-2 size-4" />
          {{ t('command.actions.uploadFiles') }}
        </CommandItem>
        <CommandItem value="refresh" @select="runAction(() => browserStore.refresh())">
          <RefreshCw class="mr-2 size-4" />
          {{ t('command.actions.refresh') }}
        </CommandItem>
        <CommandItem
          v-if="browserStore.currentPrefix"
          value="go-up"
          @select="runAction(() => browserStore.navigateUp())"
        >
          <ArrowUp class="mr-2 size-4" />
          {{ t('command.actions.goUp') }}
        </CommandItem>
      </CommandGroup>

      <CommandSeparator />

      <CommandGroup :heading="t('command.groups.connection')">
        <CommandItem
          value="disconnect"
          @select="runAction(() => { connectionStore.clearConnection(); router.push('/auth') })"
        >
          <LogOut class="mr-2 size-4" />
          {{ t('common.disconnect') }}
        </CommandItem>
      </CommandGroup>
    </CommandList>
  </CommandDialog>
</template>
