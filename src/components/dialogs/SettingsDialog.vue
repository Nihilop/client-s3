<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { useI18n } from 'vue-i18n'
import { setLocale, getLocale } from '@/i18n'
import type { ConnectionInfo } from '@/stores/connection'
import {
  Dialog,
  DialogContent,
  DialogTitle,
  DialogDescription,
} from '@/components/ui/dialog'
import {
  Sidebar,
  SidebarContent,
  SidebarGroup,
  SidebarGroupContent,
  SidebarMenu,
  SidebarMenuButton,
  SidebarMenuItem,
  SidebarProvider,
} from '@/components/ui/sidebar'
import {
  Breadcrumb,
  BreadcrumbItem,
  BreadcrumbLink,
  BreadcrumbList,
  BreadcrumbPage,
  BreadcrumbSeparator,
} from '@/components/ui/breadcrumb'
import {
  AlertDialog,
  AlertDialogAction,
  AlertDialogCancel,
  AlertDialogContent,
  AlertDialogDescription,
  AlertDialogFooter,
  AlertDialogHeader,
  AlertDialogTitle,
} from '@/components/ui/alert-dialog'
import { Button } from '@/components/ui/button'
import { Label } from '@/components/ui/label'
import { ScrollArea } from '@/components/ui/scroll-area'
import { Separator } from '@/components/ui/separator'
import {
  Settings,
  Users,
  Trash2,
  Sun,
  Moon,
  Monitor,
  Globe,
} from 'lucide-vue-next'
import { toast } from 'vue-sonner'

const { t } = useI18n()
const open = defineModel<boolean>('open', { required: true })

const activeSection = ref('general')
const currentLocale = ref(getLocale())
const currentTheme = ref(localStorage.getItem('theme') ?? 'system')

// Sessions
const profiles = ref<ConnectionInfo[]>([])
const deleteConfirmId = ref<string | null>(null)

const nav = [
  { id: 'general', icon: Settings },
  { id: 'sessions', icon: Users },
]

onMounted(async () => {
  await loadProfiles()
})

async function loadProfiles() {
  try {
    profiles.value = await invoke<ConnectionInfo[]>('list_connections')
  } catch {
    profiles.value = []
  }
}

function applyTheme(theme: string) {
  currentTheme.value = theme
  localStorage.setItem('theme', theme)
  const root = document.documentElement

  if (theme === 'system') {
    const prefersDark = window.matchMedia('(prefers-color-scheme: dark)').matches
    root.classList.toggle('dark', prefersDark)
  } else {
    root.classList.toggle('dark', theme === 'dark')
  }
}

function changeLocale(locale: string) {
  currentLocale.value = locale
  setLocale(locale)
}

async function deleteProfile(id: string) {
  try {
    await invoke('delete_connection', { id })
    await loadProfiles()
    toast.success(t('settings.sessions.deleted'))
  } catch (e: any) {
    toast.error(typeof e === 'string' ? e : e.message)
  }
  deleteConfirmId.value = null
}
</script>

<template>
  <Dialog v-model:open="open">
    <DialogContent class="overflow-hidden p-0 md:max-h-[500px] md:max-w-[700px] lg:max-w-[800px]">
      <DialogTitle class="sr-only">{{ t('settings.title') }}</DialogTitle>
      <DialogDescription class="sr-only">{{ t('settings.title') }}</DialogDescription>

      <SidebarProvider class="items-start">
        <Sidebar collapsible="none" class="hidden md:flex">
          <SidebarContent>
            <SidebarGroup>
              <SidebarGroupContent>
                <SidebarMenu>
                  <SidebarMenuItem v-for="item in nav" :key="item.id">
                    <SidebarMenuButton
                      :is-active="activeSection === item.id"
                      @click="activeSection = item.id"
                    >
                      <component :is="item.icon" />
                      <span>{{ t(`settings.tabs.${item.id}`) }}</span>
                    </SidebarMenuButton>
                  </SidebarMenuItem>
                </SidebarMenu>
              </SidebarGroupContent>
            </SidebarGroup>
          </SidebarContent>
        </Sidebar>

        <main class="flex h-[480px] flex-1 flex-col overflow-hidden">
          <header class="flex h-12 shrink-0 items-center gap-2 border-b border-border">
            <div class="flex items-center gap-2 px-4">
              <Breadcrumb>
                <BreadcrumbList>
                  <BreadcrumbItem class="hidden md:block">
                    <BreadcrumbLink class="cursor-default">{{ t('settings.title') }}</BreadcrumbLink>
                  </BreadcrumbItem>
                  <BreadcrumbSeparator class="hidden md:block" />
                  <BreadcrumbItem>
                    <BreadcrumbPage>{{ t(`settings.tabs.${activeSection}`) }}</BreadcrumbPage>
                  </BreadcrumbItem>
                </BreadcrumbList>
              </Breadcrumb>
            </div>
          </header>

          <ScrollArea class="flex-1">
            <div class="flex flex-col gap-6 p-6">
              <!-- General -->
              <template v-if="activeSection === 'general'">
                <!-- Theme -->
                <div class="grid gap-2">
                  <Label class="text-sm font-medium">{{ t('settings.general.theme') }}</Label>
                  <p class="text-xs text-muted-foreground">{{ t('settings.general.themeDescription') }}</p>
                  <div class="mt-1 flex gap-2">
                    <Button
                      v-for="theme in ['light', 'dark', 'system']"
                      :key="theme"
                      :variant="currentTheme === theme ? 'default' : 'outline'"
                      size="sm"
                      class="h-8 gap-1.5 text-xs"
                      @click="applyTheme(theme)"
                    >
                      <Sun v-if="theme === 'light'" class="size-3.5" />
                      <Moon v-else-if="theme === 'dark'" class="size-3.5" />
                      <Monitor v-else class="size-3.5" />
                      {{ t(`settings.general.theme${theme.charAt(0).toUpperCase() + theme.slice(1)}`) }}
                    </Button>
                  </div>
                </div>

                <Separator />

                <!-- Language -->
                <div class="grid gap-2">
                  <Label class="text-sm font-medium">{{ t('settings.general.language') }}</Label>
                  <p class="text-xs text-muted-foreground">{{ t('settings.general.languageDescription') }}</p>
                  <div class="mt-1 flex gap-2">
                    <Button
                      :variant="currentLocale === 'fr' ? 'default' : 'outline'"
                      size="sm"
                      class="h-8 gap-1.5 text-xs"
                      @click="changeLocale('fr')"
                    >
                      <Globe class="size-3.5" />
                      {{ t('settings.general.langFr') }}
                    </Button>
                    <Button
                      :variant="currentLocale === 'en' ? 'default' : 'outline'"
                      size="sm"
                      class="h-8 gap-1.5 text-xs"
                      @click="changeLocale('en')"
                    >
                      <Globe class="size-3.5" />
                      {{ t('settings.general.langEn') }}
                    </Button>
                  </div>
                </div>
              </template>

              <!-- Sessions -->
              <template v-if="activeSection === 'sessions'">
                <div class="grid gap-2">
                  <Label class="text-sm font-medium">{{ t('settings.sessions.title') }}</Label>
                  <p class="text-xs text-muted-foreground">{{ t('settings.sessions.description') }}</p>
                </div>

                <div v-if="profiles.length === 0" class="py-8 text-center text-sm text-muted-foreground">
                  {{ t('settings.sessions.empty') }}
                </div>

                <div v-else class="grid gap-2">
                  <div
                    v-for="profile in profiles"
                    :key="profile.id"
                    class="flex items-center justify-between rounded-md border px-3 py-2.5"
                  >
                    <div class="flex min-w-0 flex-1 flex-col">
                      <span class="truncate text-sm font-medium">{{ profile.name }}</span>
                      <span class="truncate text-[11px] text-muted-foreground">
                        {{ profile.endpoint }} · {{ profile.region }}
                      </span>
                    </div>
                    <Button
                      variant="ghost"
                      size="icon"
                      class="ml-2 size-7 shrink-0 text-muted-foreground hover:text-destructive"
                      @click="deleteConfirmId = profile.id"
                    >
                      <Trash2 class="size-3.5" />
                    </Button>
                  </div>
                </div>
              </template>
            </div>
          </ScrollArea>
        </main>
      </SidebarProvider>
    </DialogContent>
  </Dialog>

  <!-- Delete confirm -->
  <AlertDialog :open="deleteConfirmId !== null">
    <AlertDialogContent>
      <AlertDialogHeader>
        <AlertDialogTitle>{{ t('settings.sessions.deleteTitle') }}</AlertDialogTitle>
        <AlertDialogDescription>{{ t('settings.sessions.deleteDescription') }}</AlertDialogDescription>
      </AlertDialogHeader>
      <AlertDialogFooter>
        <AlertDialogCancel @click="deleteConfirmId = null">{{ t('common.cancel') }}</AlertDialogCancel>
        <AlertDialogAction class="bg-destructive hover:bg-destructive/90" @click="deleteProfile(deleteConfirmId!)">
          {{ t('common.delete') }}
        </AlertDialogAction>
      </AlertDialogFooter>
    </AlertDialogContent>
  </AlertDialog>
</template>
