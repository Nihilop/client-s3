<script setup lang="ts">
import ObjectBrowser from '@/components/browser/ObjectBrowser.vue'

const emit = defineEmits<{
  createFolder: []
  upload: []
}>()
</script>

<template>
  <ObjectBrowser
    class="h-full"
    @create-folder="emit('createFolder')"
    @upload="emit('upload')"
  />
</template>
