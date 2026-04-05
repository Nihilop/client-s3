import { ref, shallowRef } from 'vue'
import { check, type Update } from '@tauri-apps/plugin-updater'
import { relaunch } from '@tauri-apps/plugin-process'
import { toast } from 'vue-sonner'

const updateAvailable = shallowRef<Update | null>(null)
const checking = ref(false)
const downloading = ref(false)
const progress = ref(0)

export function useUpdater() {
  async function checkForUpdate() {
    if (checking.value) return
    checking.value = true
    try {
      const update = await check()
      if (update) {
        updateAvailable.value = update
      }
    } catch {
      // Silently fail — no network, etc.
    } finally {
      checking.value = false
    }
  }

  async function installUpdate() {
    const update = updateAvailable.value
    if (!update || downloading.value) return
    downloading.value = true
    progress.value = 0

    try {
      let totalLength = 0
      let downloaded = 0

      await update.downloadAndInstall((event) => {
        if (event.event === 'Started' && event.data.contentLength) {
          totalLength = event.data.contentLength
        } else if (event.event === 'Progress') {
          downloaded += event.data.chunkLength
          if (totalLength > 0) {
            progress.value = Math.round((downloaded / totalLength) * 100)
          }
        }
      })

      await relaunch()
    } catch (e: any) {
      downloading.value = false
      toast.error(typeof e === 'string' ? e : e.message ?? 'Update failed')
    }
  }

  function dismiss() {
    updateAvailable.value = null
  }

  return {
    updateAvailable,
    checking,
    downloading,
    progress,
    checkForUpdate,
    installUpdate,
    dismiss,
  }
}
