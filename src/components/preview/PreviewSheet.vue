<script setup lang="ts">
import { ref, watch, computed } from 'vue'
import { useS3, type PreviewData } from '@/composables/useS3'
import { useI18n } from 'vue-i18n'
import { save } from '@tauri-apps/plugin-dialog'
import {
  Sheet,
  SheetContent,
  SheetHeader,
  SheetTitle,
  SheetDescription,
} from '@/components/ui/sheet'
import { Button } from '@/components/ui/button'
import { Badge } from '@/components/ui/badge'
import { ScrollArea } from '@/components/ui/scroll-area'
import { Spinner } from '@/components/ui/spinner'
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs'
import {
  Download,
  Music,
  File,
  Info,
  Link,
  Eye,
} from 'lucide-vue-next'
import { toast } from 'vue-sonner'

const { t, locale } = useI18n()

const props = defineProps<{
  bucket: string
  objectKey: string
}>()

const open = defineModel<boolean>('open', { required: true })

const s3 = useS3()
const loading = ref(true)
const preview = ref<PreviewData | null>(null)
const presignedUrl = ref('')
const metadata = ref<Record<string, string> | null>(null)
const activeTab = ref('preview')

const fileName = computed(() => {
  return props.objectKey.split('/').pop() ?? props.objectKey
})

const fileExt = computed(() => {
  return fileName.value.split('.').pop()?.toLowerCase() ?? ''
})

const category = computed((): 'image' | 'video' | 'audio' | 'text' | 'code' | 'pdf' | 'unknown' => {
  if (!preview.value) return 'unknown'
  const ct = preview.value.content_type

  if (ct.startsWith('image/')) return 'image'
  if (ct.startsWith('video/')) return 'video'
  if (ct.startsWith('audio/')) return 'audio'
  if (ct === 'application/pdf') return 'pdf'

  const codeExts = ['js', 'ts', 'vue', 'jsx', 'tsx', 'py', 'rs', 'go', 'java', 'rb', 'php', 'c', 'cpp', 'h', 'css', 'scss', 'html', 'xml', 'json', 'yaml', 'yml', 'toml', 'ini', 'sh', 'bash', 'sql', 'graphql', 'md', 'mdx', 'svelte', 'astro']
  if (codeExts.includes(fileExt.value)) return 'code'

  if (ct.startsWith('text/') || ct === 'application/json' || ct === 'application/xml' || ct === 'application/javascript') return 'text'

  return 'unknown'
})

const textContent = computed(() => {
  if (!preview.value?.data) return ''
  try {
    return atob(preview.value.data)
  } catch {
    return ''
  }
})

const dataUrl = computed(() => {
  if (!preview.value?.data) return ''
  return `data:${preview.value.content_type};base64,${preview.value.data}`
})

function formatSize(bytes: number): string {
  if (bytes === 0) return '0 o'
  const units = locale.value === 'fr' ? ['o', 'Ko', 'Mo', 'Go', 'To'] : ['B', 'KB', 'MB', 'GB', 'TB']
  const i = Math.floor(Math.log(bytes) / Math.log(1024))
  return `${(bytes / Math.pow(1024, i)).toFixed(i > 0 ? 1 : 0)} ${units[i]}`
}

watch(open, async (isOpen) => {
  if (!isOpen) return
  loading.value = true
  preview.value = null
  presignedUrl.value = ''
  metadata.value = null
  activeTab.value = 'preview'

  try {
    // Load preview data and presigned URL in parallel
    const [previewData, url, details] = await Promise.all([
      s3.getObjectPreview(props.bucket, props.objectKey, 10 * 1024 * 1024),
      s3.getPresignedUrl(props.bucket, props.objectKey),
      s3.headObject(props.bucket, props.objectKey),
    ])

    preview.value = previewData
    presignedUrl.value = url
    metadata.value = {
      [t('preview.meta.contentType')]: details.content_type ?? t('common.unknown'),
      [t('preview.meta.size')]: formatSize(details.size),
      [t('preview.meta.lastModified')]: details.last_modified ?? '-',
      [t('preview.meta.etag')]: details.etag ?? '-',
      [t('preview.meta.storageClass')]: details.storage_class ?? '-',
      ...details.metadata,
    }
  } catch (e: any) {
    toast.error(typeof e === 'string' ? e : e.message)
    open.value = false
  } finally {
    loading.value = false
  }
})

async function handleDownload() {
  const destPath = await save({ defaultPath: fileName.value })
  if (!destPath) return
  try {
    await s3.downloadFile(props.bucket, props.objectKey, destPath)
    toast.success(t('preview.downloaded', { name: fileName.value }))
  } catch (e: any) {
    toast.error(typeof e === 'string' ? e : e.message)
  }
}

async function copyUrl() {
  try {
    await navigator.clipboard.writeText(presignedUrl.value)
    toast.success(t('preview.urlCopied'))
  } catch {
    toast.error(t('preview.copyError'))
  }
}
</script>

<template>
  <Sheet v-model:open="open">
    <SheetContent class="flex w-full flex-col sm:max-w-2xl p-5">
      <SheetHeader class="shrink-0">
        <div class="flex items-center gap-2">
          <SheetTitle class="truncate text-sm">{{ fileName }}</SheetTitle>
          <Badge v-if="preview" variant="secondary" class="shrink-0 text-[10px]">
            {{ preview.content_type }}
          </Badge>
        </div>
        <SheetDescription v-if="preview" class="text-xs">
          {{ formatSize(preview.size) }}
          <span class="mx-1">·</span>
          {{ props.objectKey }}
        </SheetDescription>
      </SheetHeader>

      <!-- Loading -->
      <div v-if="loading" class="flex flex-1 items-center justify-center">
        <Spinner class="size-6" />
      </div>

      <!-- Content -->
      <template v-else-if="preview">
        <!-- Actions -->
        <div class="flex shrink-0 gap-2 pb-2">
          <Button size="sm" variant="outline" class="h-7 text-xs" @click="handleDownload">
            <Download class="mr-1.5 size-3" />
            {{ t('preview.download') }}
          </Button>
          <Button size="sm" variant="outline" class="h-7 text-xs" @click="copyUrl">
            <Link class="mr-1.5 size-3" />
            {{ t('preview.copyUrl') }}
          </Button>
        </div>

        <Tabs v-model="activeTab" class="flex min-h-0 flex-1 flex-col">
          <TabsList class="w-full shrink-0">
            <TabsTrigger value="preview" class="flex-1">
              <Eye class="mr-1.5 size-3" />
              {{ t('preview.tabs.preview') }}
            </TabsTrigger>
            <TabsTrigger value="info" class="flex-1">
              <Info class="mr-1.5 size-3" />
              {{ t('preview.tabs.info') }}
            </TabsTrigger>
          </TabsList>

          <!-- Preview tab -->
          <TabsContent value="preview" class="min-h-0 flex-1">
            <ScrollArea class="h-full rounded-md border">
              <!-- Too large -->
              <div v-if="preview.too_large" class="flex h-48 flex-col items-center justify-center gap-2 p-6 text-sm text-muted-foreground">
                <File class="size-10 opacity-30" />
                <p>{{ t('preview.tooLarge') }}</p>
                <p class="text-xs">{{ formatSize(preview.size) }}</p>
                <Button size="sm" variant="outline" class="mt-2" @click="handleDownload">
                  <Download class="mr-1.5 size-3.5" />
                  {{ t('preview.download') }}
                </Button>
              </div>

              <!-- Image -->
              <div v-else-if="category === 'image'" class="flex items-center justify-center bg-muted/30 p-4">
                <img
                  :src="dataUrl"
                  :alt="fileName"
                  class="max-h-[70vh] max-w-full rounded object-contain"
                />
              </div>

              <!-- Video -->
              <div v-else-if="category === 'video'" class="flex items-center justify-center bg-black p-4">
                <video
                  :src="presignedUrl"
                  controls
                  class="max-h-[70vh] max-w-full rounded"
                >
                  {{ t('preview.videoUnsupported') }}
                </video>
              </div>

              <!-- Audio -->
              <div v-else-if="category === 'audio'" class="flex flex-col items-center justify-center gap-4 p-8">
                <Music class="size-16 text-muted-foreground opacity-30" />
                <audio :src="presignedUrl" controls class="w-full max-w-md">
                  {{ t('preview.audioUnsupported') }}
                </audio>
              </div>

              <!-- PDF -->
              <div v-else-if="category === 'pdf'" class="h-full min-h-[70vh]">
                <iframe
                  :src="presignedUrl"
                  class="size-full rounded border-0"
                  :title="t('preview.pdfTitle')"
                />
              </div>

              <!-- Code / Text -->
              <div v-else-if="category === 'code' || category === 'text'" class="p-0">
                <pre class="overflow-x-auto p-4 text-xs leading-relaxed"><code>{{ textContent }}</code></pre>
              </div>

              <!-- Unknown -->
              <div v-else class="flex h-48 flex-col items-center justify-center gap-2 p-6 text-sm text-muted-foreground">
                <File class="size-10 opacity-30" />
                <p>{{ t('preview.unknownType') }}</p>
                <p class="text-xs">{{ preview.content_type }}</p>
                <Button size="sm" variant="outline" class="mt-2" @click="handleDownload">
                  <Download class="mr-1.5 size-3.5" />
                  {{ t('preview.download') }}
                </Button>
              </div>
            </ScrollArea>
          </TabsContent>

          <!-- Info tab -->
          <TabsContent value="info" class="min-h-0 flex-1">
            <ScrollArea class="h-full rounded-md border">
              <div class="p-4">
                <dl v-if="metadata" class="grid gap-3">
                  <div v-for="(value, key) in metadata" :key="key" class="grid gap-0.5">
                    <dt class="text-[11px] font-medium uppercase tracking-wider text-muted-foreground">{{ key }}</dt>
                    <dd class="break-all text-sm">{{ value }}</dd>
                  </div>
                </dl>
              </div>
            </ScrollArea>
          </TabsContent>
        </Tabs>
      </template>
    </SheetContent>
  </Sheet>
</template>
