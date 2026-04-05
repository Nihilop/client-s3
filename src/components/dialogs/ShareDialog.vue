<script setup lang="ts">
import { ref, watch, computed } from 'vue'
import { useS3 } from '@/composables/useS3'
import { useI18n } from 'vue-i18n'
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
  DialogDescription,
} from '@/components/ui/dialog'
import { Button } from '@/components/ui/button'
import { Input } from '@/components/ui/input'
import { Spinner } from '@/components/ui/spinner'
import { Link, Copy, Check, Clock } from 'lucide-vue-next'
import { toast } from 'vue-sonner'

const { t } = useI18n()
const s3 = useS3()

const props = defineProps<{
  open: boolean
  bucket: string
  objectKey: string
}>()

const emit = defineEmits<{
  'update:open': [value: boolean]
}>()

const dialogOpen = computed({
  get: () => props.open,
  set: (v: boolean) => emit('update:open', v),
})

const fileName = computed(() => {
  const parts = props.objectKey.split('/').filter(Boolean)
  return parts[parts.length - 1] ?? props.objectKey
})

const DURATION_PRESETS = [
  { label: '15min', seconds: 900 },
  { label: '1h', seconds: 3600 },
  { label: '6h', seconds: 21600 },
  { label: '24h', seconds: 86400 },
  { label: '7d', seconds: 604800 },
  { label: '30d', seconds: 2592000 },
] as const

const selectedDuration = ref(3600)
const generatedUrl = ref('')
const loading = ref(false)
const copied = ref(false)

const expiresLabel = computed(() => {
  const preset = DURATION_PRESETS.find(p => p.seconds === selectedDuration.value)
  if (preset) {
    return t('share.expiresIn', { duration: t(`share.presets.${preset.label}`) })
  }
  return ''
})

async function generateUrl() {
  loading.value = true
  generatedUrl.value = ''
  try {
    generatedUrl.value = await s3.getPresignedUrl(props.bucket, props.objectKey, selectedDuration.value)
  } catch (e: any) {
    toast.error(typeof e === 'string' ? e : e.message)
  } finally {
    loading.value = false
  }
}

watch(dialogOpen, (v) => {
  if (v) {
    copied.value = false
    generateUrl()
  }
})

watch(selectedDuration, () => {
  if (dialogOpen.value) {
    copied.value = false
    generateUrl()
  }
})

async function copyUrl() {
  if (!generatedUrl.value) return
  try {
    await navigator.clipboard.writeText(generatedUrl.value)
    copied.value = true
    toast.success(t('share.copied'))
    setTimeout(() => {
      copied.value = false
    }, 2000)
  } catch (e: any) {
    toast.error(typeof e === 'string' ? e : e.message)
  }
}
</script>

<template>
  <Dialog v-model:model-value="dialogOpen">
    <DialogContent class="sm:max-w-lg">
      <DialogHeader>
        <DialogTitle class="flex items-center gap-2">
          <Link class="size-4" />
          {{ t('share.title') }}
        </DialogTitle>
        <DialogDescription>
          {{ t('share.description') }}
        </DialogDescription>
      </DialogHeader>

      <div class="space-y-4">
        <!-- File name -->
        <div class="text-sm font-medium truncate">{{ fileName }}</div>

        <!-- Duration presets -->
        <div class="space-y-2">
          <div class="flex items-center gap-1.5 text-xs text-muted-foreground">
            <Clock class="size-3.5" />
            {{ t('share.duration') }}
          </div>
          <div class="flex flex-wrap gap-2">
            <Button
              v-for="preset in DURATION_PRESETS"
              :key="preset.label"
              :variant="selectedDuration === preset.seconds ? 'default' : 'outline'"
              size="sm"
              @click="selectedDuration = preset.seconds"
            >
              {{ t(`share.presets.${preset.label}`) }}
            </Button>
          </div>
        </div>

        <!-- URL display -->
        <div class="space-y-2">
          <div v-if="loading" class="flex items-center justify-center py-4">
            <Spinner class="size-5 text-muted-foreground" />
            <span class="ml-2 text-sm text-muted-foreground">{{ t('share.generating') }}</span>
          </div>
          <div v-else class="flex gap-2">
            <Input
              :model-value="generatedUrl"
              readonly
              class="flex-1 text-xs font-mono"
            />
            <Button
              variant="outline"
              size="icon"
              :disabled="!generatedUrl"
              @click="copyUrl"
            >
              <Check v-if="copied" class="size-4 text-green-500" />
              <Copy v-else class="size-4" />
            </Button>
          </div>

          <!-- Expiration note -->
          <p v-if="!loading && generatedUrl" class="text-xs text-muted-foreground">
            {{ expiresLabel }}
          </p>
        </div>
      </div>
    </DialogContent>
  </Dialog>
</template>
