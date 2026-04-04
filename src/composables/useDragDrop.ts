import { ref, onMounted, onUnmounted } from 'vue'
import { getCurrentWebview } from '@tauri-apps/api/webview'

export function useDragDrop(onDrop: (paths: string[]) => void) {
  const isDragging = ref(false)
  let unlisten: (() => void) | undefined

  onMounted(async () => {
    const webview = getCurrentWebview()
    unlisten = await webview.onDragDropEvent((event) => {
      if (event.payload.type === 'over') {
        isDragging.value = true
      } else if (event.payload.type === 'drop') {
        isDragging.value = false
        if (event.payload.paths.length > 0) {
          onDrop(event.payload.paths)
        }
      } else {
        // cancel
        isDragging.value = false
      }
    })
  })

  onUnmounted(() => {
    unlisten?.()
  })

  return { isDragging }
}
