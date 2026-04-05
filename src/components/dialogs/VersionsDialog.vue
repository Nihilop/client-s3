<script setup lang="ts">
import { ref, watch } from 'vue'
import { useS3, type ObjectVersion } from '@/composables/useS3'
import { useI18n } from 'vue-i18n'
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
  DialogDescription,
} from '@/components/ui/dialog'
import { Button } from '@/components/ui/button'
import { Badge } from '@/components/ui/badge'
import { ScrollArea } from '@/components/ui/scroll-area'
import { Spinner } from '@/components/ui/spinner'
import { RotateCcw, Trash2 } from 'lucide-vue-next'
import { toast } from 'vue-sonner'

const { t, locale } = useI18n()
const s3 = useS3()

const open = defineModel<boolean>('open', { required: true })

const props = defineProps<{
  bucket: string
  objectKey: string
}>()

const loading = ref(false)
const versions = ref<ObjectVersion[]>([])
const error = ref('')

async function loadVersions() {
  loading.value = true
  error.value = ''
  versions.value = []
  try {
    versions.value = await s3.listObjectVersions(props.bucket, props.objectKey)
  } catch (e: any) {
    error.value = typeof e === 'string' ? e : e.message ?? 'Error'
  } finally {
    loading.value = false
  }
}

watch(open, (v) => {
  if (v) loadVersions()
})

async function restoreVersion(version: ObjectVersion) {
  if (!version.version_id) return
  try {
    await s3.restoreObjectVersion(props.bucket, props.objectKey, version.version_id)
    toast.success(t('versions.restored'))
    await loadVersions()
  } catch (e: any) {
    toast.error(typeof e === 'string' ? e : e.message)
  }
}

async function deleteVersion(version: ObjectVersion) {
  if (!version.version_id) return
  try {
    await s3.deleteObjectVersion(props.bucket, props.objectKey, version.version_id)
    toast.success(t('versions.deleted'))
    await loadVersions()
  } catch (e: any) {
    toast.error(typeof e === 'string' ? e : e.message)
  }
}

function truncateVersionId(id: string | null): string {
  if (!id) return '-'
  if (id.length <= 16) return id
  return id.slice(0, 8) + '...' + id.slice(-8)
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

function fileName(): string {
  return props.objectKey.split('/').filter(Boolean).pop() ?? props.objectKey
}
</script>

<template>
  <Dialog v-model:open="open">
    <DialogContent class="sm:max-w-2xl">
      <DialogHeader>
        <DialogTitle>{{ t('versions.title') }}</DialogTitle>
        <DialogDescription>{{ t('versions.description', { name: fileName() }) }}</DialogDescription>
      </DialogHeader>

      <div v-if="loading" class="flex items-center justify-center py-8">
        <Spinner class="size-6 text-muted-foreground" />
      </div>

      <div v-else-if="error" class="py-4 text-sm text-destructive">
        {{ error }}
      </div>

      <div v-else-if="versions.length === 0" class="py-8 text-center text-sm text-muted-foreground">
        {{ t('versions.empty') }}
      </div>

      <ScrollArea v-else class="max-h-[400px]">
        <!-- Header -->
        <div class="sticky top-0 z-10 grid grid-cols-[1fr_140px_80px_auto] gap-2 border-b border-border bg-muted/50 px-3 py-1.5 text-xs font-medium text-muted-foreground">
          <span>{{ t('versions.versionId') }}</span>
          <span>{{ t('versions.date') }}</span>
          <span class="text-right">{{ t('versions.size') }}</span>
          <span class="w-[72px]" />
        </div>

        <!-- Version rows -->
        <div
          v-for="version in versions"
          :key="version.version_id ?? 'null'"
          class="grid grid-cols-[1fr_140px_80px_auto] items-center gap-2 border-b border-border/50 px-3 py-2 text-sm"
        >
          <div class="flex items-center gap-2 overflow-hidden">
            <span class="truncate font-mono text-xs">{{ truncateVersionId(version.version_id) }}</span>
            <Badge v-if="version.is_latest" variant="default" class="text-[10px]">
              {{ t('versions.latest') }}
            </Badge>
            <Badge v-if="version.is_delete_marker" variant="destructive" class="text-[10px]">
              {{ t('versions.deleteMarker') }}
            </Badge>
          </div>

          <span class="text-xs text-muted-foreground">{{ formatDate(version.last_modified) }}</span>

          <span class="text-right text-xs text-muted-foreground">
            {{ version.is_delete_marker ? '-' : formatSize(version.size) }}
          </span>

          <div class="flex items-center gap-1">
            <Button
              v-if="!version.is_latest && !version.is_delete_marker"
              variant="ghost"
              size="icon"
              class="size-7"
              :title="t('versions.restore')"
              @click="restoreVersion(version)"
            >
              <RotateCcw class="size-3.5" />
            </Button>
            <Button
              v-if="!version.is_latest"
              variant="ghost"
              size="icon"
              class="size-7 text-destructive hover:text-destructive"
              :title="t('versions.delete')"
              @click="deleteVersion(version)"
            >
              <Trash2 class="size-3.5" />
            </Button>
          </div>
        </div>
      </ScrollArea>
    </DialogContent>
  </Dialog>
</template>
