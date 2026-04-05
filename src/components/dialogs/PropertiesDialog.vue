<script setup lang="ts">
import { ref, watch } from 'vue'
import { useS3, type ObjectDetails } from '@/composables/useS3'
import { useI18n } from 'vue-i18n'
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
} from '@/components/ui/dialog'
import { Separator } from '@/components/ui/separator'
import { Spinner } from '@/components/ui/spinner'

const { t, locale } = useI18n()
const s3 = useS3()

const open = defineModel<boolean>('open', { required: true })

const props = defineProps<{
  bucket: string
  objectKey: string
}>()

const loading = ref(false)
const details = ref<ObjectDetails | null>(null)
const error = ref('')

watch(open, async (v) => {
  if (!v) return
  loading.value = true
  error.value = ''
  details.value = null
  try {
    details.value = await s3.headObject(props.bucket, props.objectKey)
  } catch (e: any) {
    error.value = typeof e === 'string' ? e : e.message ?? 'Error'
  } finally {
    loading.value = false
  }
})

function formatSize(bytes: number): string {
  if (bytes === 0) return '0 B'
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
      second: '2-digit',
    })
  } catch {
    return date
  }
}
</script>

<template>
  <Dialog v-model:open="open">
    <DialogContent class="sm:max-w-lg">
      <DialogHeader>
        <DialogTitle>{{ t('properties.title') }}</DialogTitle>
      </DialogHeader>

      <div v-if="loading" class="flex items-center justify-center py-8">
        <Spinner class="size-6 text-muted-foreground" />
      </div>

      <div v-else-if="error" class="py-4 text-sm text-destructive">
        {{ error }}
      </div>

      <div v-else-if="details" class="grid gap-3 text-sm">
        <!-- Key/Path -->
        <div class="grid grid-cols-[140px_1fr] gap-2">
          <span class="font-medium text-muted-foreground">{{ t('properties.key') }}</span>
          <span class="break-all">{{ details.key }}</span>
        </div>

        <!-- Size -->
        <div class="grid grid-cols-[140px_1fr] gap-2">
          <span class="font-medium text-muted-foreground">{{ t('properties.size') }}</span>
          <span>{{ formatSize(details.size) }}</span>
        </div>

        <!-- Content-Type -->
        <div class="grid grid-cols-[140px_1fr] gap-2">
          <span class="font-medium text-muted-foreground">{{ t('properties.contentType') }}</span>
          <span>{{ details.content_type ?? '-' }}</span>
        </div>

        <!-- Last Modified -->
        <div class="grid grid-cols-[140px_1fr] gap-2">
          <span class="font-medium text-muted-foreground">{{ t('properties.lastModified') }}</span>
          <span>{{ formatDate(details.last_modified) }}</span>
        </div>

        <!-- ETag -->
        <div class="grid grid-cols-[140px_1fr] gap-2">
          <span class="font-medium text-muted-foreground">{{ t('properties.etag') }}</span>
          <span class="break-all font-mono text-xs">{{ details.etag ?? '-' }}</span>
        </div>

        <!-- Storage Class -->
        <div class="grid grid-cols-[140px_1fr] gap-2">
          <span class="font-medium text-muted-foreground">{{ t('properties.storageClass') }}</span>
          <span>{{ details.storage_class ?? '-' }}</span>
        </div>

        <Separator />

        <!-- Custom Metadata -->
        <div>
          <span class="font-medium text-muted-foreground">{{ t('properties.metadata') }}</span>
          <div v-if="Object.keys(details.metadata).length === 0" class="mt-2 text-muted-foreground text-xs">
            {{ t('properties.noMetadata') }}
          </div>
          <div v-else class="mt-2 rounded border border-border">
            <div
              v-for="(value, key) in details.metadata"
              :key="key"
              class="grid grid-cols-[1fr_2fr] gap-2 border-b border-border/50 px-3 py-1.5 text-xs last:border-b-0"
            >
              <span class="font-medium font-mono">{{ key }}</span>
              <span class="break-all">{{ value }}</span>
            </div>
          </div>
        </div>
      </div>
    </DialogContent>
  </Dialog>
</template>
