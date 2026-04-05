import { ref, computed } from 'vue'
import { listen } from '@tauri-apps/api/event'

export type TransferStatus = 'InProgress' | 'Completed' | { Failed: string } | 'Cancelled' | 'Pending'

export interface Transfer {
  id: string
  fileName: string
  bytesTransferred: number
  totalBytes: number
  status: TransferStatus
  progress: number
  updatedAt: number
}

interface TransferProgress {
  id: string
  file_name: string
  bytes_transferred: number
  total_bytes: number
  status: TransferStatus
}

const transferMap = ref<Map<string, Transfer>>(new Map())
let initialized = false
const removeTimers = new Map<string, ReturnType<typeof setTimeout>>()

function computeProgress(bytes: number, total: number): number {
  if (total <= 0) return 0
  return Math.min(100, Math.round((bytes / total) * 100))
}

function scheduleAutoRemove(id: string) {
  if (removeTimers.has(id)) {
    clearTimeout(removeTimers.get(id)!)
  }
  const timer = setTimeout(() => {
    transferMap.value.delete(id)
    removeTimers.delete(id)
  }, 5000)
  removeTimers.set(id, timer)
}

function isTerminalStatus(status: TransferStatus): boolean {
  return status === 'Completed' || status === 'Cancelled' || (typeof status === 'object' && 'Failed' in status)
}

export function useTransfers() {
  const transfers = computed(() => {
    return Array.from(transferMap.value.values()).sort((a, b) => b.updatedAt - a.updatedAt)
  })

  const activeCount = computed(() => {
    return Array.from(transferMap.value.values()).filter(t => t.status === 'InProgress').length
  })

  const hasActive = computed(() => activeCount.value > 0)

  function clearCompleted() {
    for (const [id, transfer] of transferMap.value) {
      if (isTerminalStatus(transfer.status)) {
        transferMap.value.delete(id)
        if (removeTimers.has(id)) {
          clearTimeout(removeTimers.get(id)!)
          removeTimers.delete(id)
        }
      }
    }
  }

  async function init() {
    if (initialized) return
    initialized = true

    await listen<TransferProgress>('transfer-progress', (event) => {
      const payload = event.payload
      const transfer: Transfer = {
        id: payload.id,
        fileName: payload.file_name,
        bytesTransferred: payload.bytes_transferred,
        totalBytes: payload.total_bytes,
        status: payload.status,
        progress: computeProgress(payload.bytes_transferred, payload.total_bytes),
        updatedAt: Date.now(),
      }

      transferMap.value.set(payload.id, transfer)

      if (isTerminalStatus(payload.status)) {
        scheduleAutoRemove(payload.id)
      }
    })
  }

  return {
    transfers,
    activeCount,
    hasActive,
    clearCompleted,
    init,
  }
}
