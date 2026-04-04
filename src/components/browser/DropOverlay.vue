<script setup lang="ts">
import { Upload } from 'lucide-vue-next'
import { useI18n } from 'vue-i18n'

const { t } = useI18n()

defineProps<{ visible: boolean }>()
</script>

<template>
  <Transition
    enter-active-class="transition-opacity duration-150"
    leave-active-class="transition-opacity duration-150"
    enter-from-class="opacity-0"
    leave-to-class="opacity-0"
  >
    <div
      v-if="visible"
      class="pointer-events-none absolute inset-0 z-50 flex items-center justify-center bg-background/80 backdrop-blur-sm"
    >
      <div class="flex flex-col items-center gap-3 rounded-xl border-2 border-dashed border-primary/50 bg-background p-10">
        <Upload class="size-10 text-primary" />
        <p class="text-sm font-medium">{{ t('dropOverlay.title') }}</p>
        <p class="text-xs text-muted-foreground">{{ t('dropOverlay.subtitle') }}</p>
      </div>
    </div>
  </Transition>
</template>
