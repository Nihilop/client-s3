<script setup lang="ts">
import { ref, watch } from 'vue'
import { useBrowserStore } from '@/stores/browser'
import { useI18n } from 'vue-i18n'
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from '@/components/ui/dialog'
import { Button } from '@/components/ui/button'
import { Input } from '@/components/ui/input'
import { Label } from '@/components/ui/label'
import { toast } from 'vue-sonner'

const { t } = useI18n()

const open = defineModel<boolean>('open', { required: true })

const store = useBrowserStore()
const name = ref('')
const loading = ref(false)

watch(open, (v) => {
  if (v) name.value = ''
})

async function submit() {
  if (!name.value.trim()) return
  loading.value = true
  try {
    await store.createBucket(name.value.trim())
    toast.success(t('browser.toast.bucketCreated', { name: name.value }))
    open.value = false
  } catch (e: any) {
    toast.error(typeof e === 'string' ? e : e.message)
  } finally {
    loading.value = false
  }
}
</script>

<template>
  <Dialog v-model:open="open">
    <DialogContent class="sm:max-w-md">
      <DialogHeader>
        <DialogTitle>{{ t('dialogs.createBucket.title') }}</DialogTitle>
        <DialogDescription>
          {{ t('dialogs.createBucket.description') }}
        </DialogDescription>
      </DialogHeader>
      <form @submit.prevent="submit">
        <div class="grid gap-4 py-4">
          <div class="grid gap-2">
            <Label for="bucket-name">{{ t('dialogs.createBucket.label') }}</Label>
            <Input
              id="bucket-name"
              v-model="name"
              :placeholder="t('dialogs.createBucket.placeholder')"
              pattern="[a-z0-9][a-z0-9.\-]{1,61}[a-z0-9]"
              required
              autofocus
            />
          </div>
        </div>
        <DialogFooter>
          <Button type="button" variant="outline" @click="open = false">
            {{ t('common.cancel') }}
          </Button>
          <Button type="submit" :disabled="loading || !name.trim()">
            {{ loading ? t('common.creating') : t('common.create') }}
          </Button>
        </DialogFooter>
      </form>
    </DialogContent>
  </Dialog>
</template>
