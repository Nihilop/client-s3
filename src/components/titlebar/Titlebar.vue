<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue'
import { getCurrentWindow } from '@tauri-apps/api/window'
import { Minus, Square, X, Maximize2 } from 'lucide-vue-next'
import { useI18n } from 'vue-i18n'

const { t } = useI18n()
const appWindow = getCurrentWindow()
const isMaximized = ref(false)

let unlisten: (() => void) | undefined

onMounted(async () => {
  isMaximized.value = await appWindow.isMaximized()
  unlisten = await appWindow.onResized(async () => {
    isMaximized.value = await appWindow.isMaximized()
  })
})

onUnmounted(() => {
  unlisten?.()
})

function minimize() {
  appWindow.minimize()
}

function toggleMaximize() {
  appWindow.toggleMaximize()
}

function close() {
  appWindow.close()
}
</script>

<template>
  <div
    data-tauri-drag-region
    class="titlebar"
  >
    <!-- Left: branding -->
    <div data-tauri-drag-region class="titlebar-brand">
      <div class="titlebar-logo">
        <span>{{ t('titlebar.logo') }}</span>
      </div>
      <span data-tauri-drag-region class="titlebar-title">
        {{ t('titlebar.title') }}
      </span>
    </div>

    <!-- Center: spacer draggable -->
    <div data-tauri-drag-region class="flex-1" />

    <!-- Right: window controls -->
    <div class="titlebar-controls">
      <button class="titlebar-btn" @click="minimize">
        <Minus class="size-3.5" :stroke-width="1.5" />
      </button>
      <button class="titlebar-btn" @click="toggleMaximize">
        <Maximize2 v-if="!isMaximized" class="size-3" :stroke-width="1.5" />
        <Square v-else class="size-3" :stroke-width="1.5" />
      </button>
      <button class="titlebar-btn titlebar-btn-close" @click="close">
        <X class="size-3.5" :stroke-width="1.5" />
      </button>
    </div>
  </div>
</template>

<style scoped>
.titlebar {
  display: flex;
  align-items: center;
  justify-content: space-between;
  height: 2.25rem;
  user-select: none;
  -webkit-user-select: none;
  border-bottom: 1px solid var(--border);
  background: var(--background);
  flex-shrink: 0;
}

.titlebar-brand {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  padding-left: 0.75rem;
}

.titlebar-logo {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 1.125rem;
  height: 1.125rem;
  border-radius: 0.25rem;
  background: var(--primary);
  font-size: 0.5625rem;
  font-weight: 700;
  color: var(--primary-foreground);
  line-height: 1;
}

.titlebar-title {
  font-size: 0.75rem;
  font-weight: 500;
  color: var(--muted-foreground);
}

.titlebar-controls {
  display: flex;
  height: 100%;
}

.titlebar-btn {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  width: 2.75rem;
  height: 100%;
  color: var(--muted-foreground);
  background: transparent;
  border: none;
  cursor: pointer;
  transition: background-color 0.15s, color 0.15s;
}

.titlebar-btn:hover {
  background: var(--accent);
  color: var(--accent-foreground);
}

.titlebar-btn-close:hover {
  background: var(--destructive);
  color: white;
}
</style>
