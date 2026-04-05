import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { invoke } from '@tauri-apps/api/core'

export interface ConnectionInfo {
  id: string
  name: string
  endpoint: string
  region: string
  access_key_id: string
  path_style: boolean
}

export const useConnectionStore = defineStore('connection', () => {
  const activeConnection = ref<ConnectionInfo | null>(null)
  const restored = ref(false)

  const isConnected = computed(() => activeConnection.value !== null)

  function setConnection(connection: ConnectionInfo) {
    activeConnection.value = connection
  }

  function clearConnection() {
    activeConnection.value = null
  }

  /** Try to restore the active session from the Rust backend (survives page reload) */
  async function restoreSession() {
    try {
      const conn = await invoke<ConnectionInfo | null>('get_active_connection')
      if (conn) {
        activeConnection.value = conn
      }
    } catch {
      // No active session on backend
    } finally {
      restored.value = true
    }
  }

  return {
    activeConnection,
    isConnected,
    restored,
    setConnection,
    clearConnection,
    restoreSession,
  }
})
