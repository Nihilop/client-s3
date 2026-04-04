import { defineStore } from 'pinia'
import { ref, computed } from 'vue'

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

  const isConnected = computed(() => activeConnection.value !== null)

  function setConnection(connection: ConnectionInfo) {
    activeConnection.value = connection
  }

  function clearConnection() {
    activeConnection.value = null
  }

  return {
    activeConnection,
    isConnected,
    setConnection,
    clearConnection,
  }
})
