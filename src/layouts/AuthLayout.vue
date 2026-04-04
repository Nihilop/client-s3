<script setup lang="ts">
import Titlebar from '@/components/titlebar/Titlebar.vue'
</script>

<template>
  <div class="flex h-screen flex-col overflow-hidden bg-background">
    <Titlebar />
    <main class="flex-1 overflow-y-auto">
      <div class="flex min-h-full items-center justify-center py-12">
        <router-view />
      </div>
    </main>
  </div>
</template>
