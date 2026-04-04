<script setup lang="ts">
import 'vue-sonner/style.css'
import { Toaster } from '@/components/ui/sonner'

// Apply saved theme on startup
const theme = localStorage.getItem('theme') ?? 'system'
if (theme === 'dark' || (theme === 'system' && window.matchMedia('(prefers-color-scheme: dark)').matches)) {
  document.documentElement.classList.add('dark')
} else {
  document.documentElement.classList.remove('dark')
}
</script>

<template>
  <Toaster position="bottom-right" :duration="4000" />
  <router-view />
</template>
