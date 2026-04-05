<script setup lang="ts">
import { ref } from 'vue'
import { useRouter } from 'vue-router'
import { useConnectionStore } from '@/stores/connection'
import { useBrowserStore } from '@/stores/browser'
import { useDragDrop } from '@/composables/useDragDrop'
import { useI18n } from 'vue-i18n'
import { useUpdater } from '@/composables/useUpdater'
import { open as openFileDialog } from '@tauri-apps/plugin-dialog'
import Titlebar from '@/components/titlebar/Titlebar.vue'
import BucketList from '@/components/browser/BucketList.vue'
import BreadcrumbNav from '@/components/browser/BreadcrumbNav.vue'
import DropOverlay from '@/components/browser/DropOverlay.vue'
import PreviewSheet from '@/components/preview/PreviewSheet.vue'
import CreateBucketDialog from '@/components/dialogs/CreateBucketDialog.vue'
import CreateFolderDialog from '@/components/dialogs/CreateFolderDialog.vue'
import SettingsDialog from '@/components/dialogs/SettingsDialog.vue'
import CommandPalette from '@/components/command/CommandPalette.vue'
import {
  Sidebar,
  SidebarContent,
  SidebarFooter,
  SidebarHeader,
  SidebarInset,
  SidebarMenu,
  SidebarMenuButton,
  SidebarMenuItem,
  SidebarProvider,
  SidebarTrigger,
} from '@/components/ui/sidebar'
import { Separator } from '@/components/ui/separator'
import { Button } from '@/components/ui/button'
import { Tooltip, TooltipContent, TooltipProvider, TooltipTrigger } from '@/components/ui/tooltip'
import { Progress } from '@/components/ui/progress'
import {
  HardDrive,
  LogOut,
  RefreshCw,
  FolderPlus,
  Upload,
  Search,
  Settings,
  ArrowDownCircle,
  X,
} from 'lucide-vue-next'
import { toast } from 'vue-sonner'

const { t } = useI18n()
const router = useRouter()
const connectionStore = useConnectionStore()
const browserStore = useBrowserStore()
const { updateAvailable, downloading, progress, checkForUpdate, installUpdate, dismiss } = useUpdater()

// Check for updates on mount (silent)
checkForUpdate()

const showCreateBucket = ref(false)
const showCreateFolder = ref(false)
const showSettings = ref(false)
const commandPaletteRef = ref<InstanceType<typeof CommandPalette> | null>(null)
const searchPreviewOpen = ref(false)
const searchPreviewBucket = ref('')
const searchPreviewKey = ref('')

function handleOpenFile(bucket: string, key: string) {
  searchPreviewBucket.value = bucket
  searchPreviewKey.value = key
  searchPreviewOpen.value = true
}

// Drag and drop
const { isDragging } = useDragDrop(async (paths) => {
  if (!browserStore.currentBucket) {
    toast.error(t('browser.selectBucketFirst'))
    return
  }
  try {
    await browserStore.uploadFiles(paths)
    toast.success(t('browser.toast.filesUploaded', { count: paths.length }))
  } catch (e: any) {
    toast.error(typeof e === 'string' ? e : e.message)
  }
})

async function handleUpload() {
  const result = await openFileDialog({ multiple: true })
  if (!result) return
  const paths = Array.isArray(result) ? result : [result]
  if (paths.length === 0) return
  if (!browserStore.currentBucket) {
    toast.error(t('browser.selectBucketFirst'))
    return
  }
  try {
    await browserStore.uploadFiles(paths)
    toast.success(t('browser.toast.filesUploaded', { count: paths.length }))
  } catch (e: any) {
    toast.error(typeof e === 'string' ? e : e.message)
  }
}

function disconnect() {
  connectionStore.clearConnection()
  router.push('/auth')
}
</script>

<template>
  <div class="flex h-screen flex-col overflow-hidden bg-background">
    <Titlebar />

    <div class="relative flex min-h-0 flex-1">
      <SidebarProvider class="flex min-h-0 flex-1">
        <Sidebar collapsible="icon">
          <SidebarHeader>
            <SidebarMenu>
              <SidebarMenuItem>
                <SidebarMenuButton size="lg" class="pointer-events-none">
                  <div class="flex size-8 items-center justify-center rounded-lg bg-primary text-primary-foreground">
                    <HardDrive class="size-4" />
                  </div>
                  <div class="grid flex-1 text-left text-sm leading-tight">
                    <span class="truncate font-semibold">
                      {{ connectionStore.activeConnection?.name ?? t('titlebar.title') }}
                    </span>
                    <span class="truncate text-xs text-muted-foreground">
                      {{ connectionStore.activeConnection?.endpoint ?? '' }}
                    </span>
                  </div>
                </SidebarMenuButton>
              </SidebarMenuItem>
            </SidebarMenu>
          </SidebarHeader>

          <SidebarContent>
            <BucketList @create-bucket="showCreateBucket = true" />
          </SidebarContent>

          <SidebarFooter>
            <!-- Update banner -->
            <div v-if="updateAvailable" class="mx-2 mb-2 rounded-md border border-primary/30 bg-primary/5 p-2.5">
              <div class="flex items-start justify-between gap-1">
                <div class="flex items-center gap-1.5">
                  <ArrowDownCircle class="size-3.5 shrink-0 text-primary" />
                  <span class="text-[11px] font-medium">{{ t('updater.version', { version: updateAvailable.version }) }}</span>
                </div>
                <button class="shrink-0 text-muted-foreground hover:text-foreground" @click="dismiss">
                  <X class="size-3" />
                </button>
              </div>
              <div v-if="downloading" class="mt-2">
                <Progress :model-value="progress" class="h-1.5" />
                <span class="mt-1 block text-[10px] text-muted-foreground">{{ t('updater.downloading', { progress }) }}</span>
              </div>
              <Button v-else size="sm" class="mt-2 h-6 w-full text-[11px]" @click="installUpdate">
                {{ t('updater.download') }}
              </Button>
            </div>

            <SidebarMenu>
              <SidebarMenuItem>
                <SidebarMenuButton @click="showSettings = true">
                  <Settings class="size-4" />
                  <span>{{ t('settings.title') }}</span>
                </SidebarMenuButton>
              </SidebarMenuItem>
              <SidebarMenuItem>
                <SidebarMenuButton @click="disconnect">
                  <LogOut class="size-4" />
                  <span>{{ t('common.disconnect') }}</span>
                </SidebarMenuButton>
              </SidebarMenuItem>
            </SidebarMenu>
          </SidebarFooter>
        </Sidebar>

        <SidebarInset class="flex min-h-0 flex-col overflow-hidden">
          <!-- Toolbar -->
          <header class="flex h-10 shrink-0 items-center gap-2 border-b border-border px-3">
            <SidebarTrigger class="-ml-1" />
            <Separator orientation="vertical" class="mr-1 h-4" />
            <BreadcrumbNav />

            <div class="ml-auto flex items-center gap-1">
              <TooltipProvider :delay-duration="300">
                <Tooltip>
                  <TooltipTrigger as-child>
                    <Button
                      variant="ghost"
                      size="icon"
                      class="size-7"
                      :disabled="!browserStore.currentBucket"
                      @click="showCreateFolder = true"
                    >
                      <FolderPlus class="size-3.5" />
                    </Button>
                  </TooltipTrigger>
                  <TooltipContent>{{ t('layout.newFolder') }}</TooltipContent>
                </Tooltip>
                <Tooltip>
                  <TooltipTrigger as-child>
                    <Button
                      variant="ghost"
                      size="icon"
                      class="size-7"
                      :disabled="!browserStore.currentBucket"
                      @click="handleUpload"
                    >
                      <Upload class="size-3.5" />
                    </Button>
                  </TooltipTrigger>
                  <TooltipContent>{{ t('layout.upload') }}</TooltipContent>
                </Tooltip>
                <Tooltip>
                  <TooltipTrigger as-child>
                    <Button variant="ghost" size="icon" class="size-7" @click="browserStore.refresh()">
                      <RefreshCw class="size-3.5" />
                    </Button>
                  </TooltipTrigger>
                  <TooltipContent>{{ t('layout.refresh') }}</TooltipContent>
                </Tooltip>
              </TooltipProvider>

              <Separator orientation="vertical" class="mx-1 h-4" />

              <Button variant="ghost" size="sm" class="h-7 gap-1.5 text-xs text-muted-foreground" @click="commandPaletteRef && (commandPaletteRef.open = true)">
                <Search class="size-3" />
                <span>{{ t('common.search') }}</span>
                <kbd class="pointer-events-none ml-1 inline-flex h-5 items-center gap-0.5 rounded border bg-muted px-1.5 font-mono text-[10px] font-medium text-muted-foreground">
                  Ctrl+K
                </kbd>
              </Button>
            </div>
          </header>

          <!-- Page content -->
          <main class="relative flex-1 overflow-hidden">
            <router-view
              @create-folder="showCreateFolder = true"
              @upload="handleUpload"
            />
            <DropOverlay :visible="isDragging" />
          </main>
        </SidebarInset>
      </SidebarProvider>
    </div>

    <!-- Dialogs -->
    <CreateBucketDialog v-model:open="showCreateBucket" />
    <CreateFolderDialog v-model:open="showCreateFolder" />
    <SettingsDialog v-model:open="showSettings" />
    <CommandPalette
      ref="commandPaletteRef"
      @create-bucket="showCreateBucket = true"
      @create-folder="showCreateFolder = true"
      @upload="handleUpload"
      @open-file="handleOpenFile"
    />
    <PreviewSheet
      v-if="searchPreviewBucket"
      v-model:open="searchPreviewOpen"
      :bucket="searchPreviewBucket"
      :object-key="searchPreviewKey"
    />
  </div>
</template>
