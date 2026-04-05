<script setup lang="ts">
import { ref, watch, onMounted, computed } from 'vue'
import { useRouter } from 'vue-router'
import { invoke } from '@tauri-apps/api/core'
import { save, open as openFileDialog } from '@tauri-apps/plugin-dialog'
import { writeTextFile, readTextFile } from '@tauri-apps/plugin-fs'
import { useConnectionStore, type ConnectionInfo } from '@/stores/connection'
import { useI18n } from 'vue-i18n'
import { providers, resolveEndpoint } from '@/lib/providers'
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from '@/components/ui/card'
import { Button } from '@/components/ui/button'
import { Input } from '@/components/ui/input'
import { Label } from '@/components/ui/label'
import { Switch } from '@/components/ui/switch'
import { Checkbox } from '@/components/ui/checkbox'
import { Spinner } from '@/components/ui/spinner'
import { Separator } from '@/components/ui/separator'
import { Badge } from '@/components/ui/badge'
import { ScrollArea } from '@/components/ui/scroll-area'
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs'
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from '@/components/ui/select'
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
import {
  HardDrive,
  Plug,
  Eye,
  EyeOff,
  Trash2,
  Download,
  Upload,
  Cloud,
  ExternalLink,
} from 'lucide-vue-next'
import { toast } from 'vue-sonner'

const { t } = useI18n()
const router = useRouter()
const connectionStore = useConnectionStore()

const loading = ref(false)
const error = ref('')
const showSecret = ref(false)
const saveProfile = ref(false)
const activeTab = ref('connect')

// Profiles
const savedProfiles = ref<ConnectionInfo[]>([])
const selectedProfileId = ref<string>('__new__')
const deleteConfirmId = ref<string | null>(null)

// Provider bridges
const selectedProvider = ref('custom')

const form = ref({
  name: '',
  endpoint: '',
  region: 'us-east-1',
  access_key_id: '',
  secret_access_key: '',
  path_style: true,
})

const currentProvider = computed(() => {
  return providers.find((p) => p.id === selectedProvider.value) ?? providers[0]
})

// Load saved profiles
onMounted(async () => {
  await loadProfiles()
})

async function loadProfiles() {
  try {
    savedProfiles.value = await invoke<ConnectionInfo[]>('list_connections')
  } catch {
    savedProfiles.value = []
  }
}

// When selecting a saved profile, populate form
watch(selectedProfileId, (id) => {
  if (!id || id === '__new__') return
  const profile = savedProfiles.value.find((p) => p.id === id)
  if (!profile) return
  form.value.name = profile.name
  form.value.endpoint = profile.endpoint
  form.value.region = profile.region
  form.value.access_key_id = profile.access_key_id
  form.value.path_style = profile.path_style
  form.value.secret_access_key = ''
  saveProfile.value = false
  error.value = ''
})

// When switching provider, auto-fill relevant fields
watch(selectedProvider, (providerId) => {
  const provider = providers.find((p) => p.id === providerId)
  if (!provider) return
  selectedProfileId.value = '__new__'

  if (provider.endpointTemplate) {
    form.value.endpoint = resolveEndpoint(provider.endpointTemplate, provider.region)
  } else {
    form.value.endpoint = provider.endpoint
  }
  form.value.region = provider.region
  form.value.path_style = provider.pathStyle

  // Clear credentials
  form.value.access_key_id = ''
  form.value.secret_access_key = ''
  form.value.name = provider.id === 'custom' ? '' : provider.name
  error.value = ''
})

// When region changes for a provider with endpoint template, update endpoint
watch(() => form.value.region, (region) => {
  const provider = currentProvider.value
  if (provider.endpointTemplate) {
    form.value.endpoint = resolveEndpoint(provider.endpointTemplate, region)
  }
})

async function handleConnect() {
  error.value = ''
  loading.value = true

  try {
    const input = {
      name: form.value.name || t('auth.connect.defaultName'),
      endpoint: form.value.endpoint,
      region: form.value.region,
      access_key_id: form.value.access_key_id,
      secret_access_key: form.value.secret_access_key,
      path_style: form.value.path_style,
    }

    let active: ConnectionInfo

    if (selectedProfileId.value && selectedProfileId.value !== '__new__') {
      // Reconnect to a saved profile — need to update it with the secret key
      await invoke('update_connection', { id: selectedProfileId.value, input })
      active = await invoke<ConnectionInfo>('connect', { id: selectedProfileId.value })
    } else if (saveProfile.value) {
      // Save and connect
      const connection = await invoke<ConnectionInfo>('save_connection', { input })
      active = await invoke<ConnectionInfo>('connect', { id: connection.id })
    } else {
      // Connect without saving
      active = await invoke<ConnectionInfo>('connect_direct', { input })
    }

    connectionStore.setConnection(active)
    router.push('/app')
  } catch (e: any) {
    error.value = typeof e === 'string' ? e : e.message ?? t('auth.connect.connectionError')
  } finally {
    loading.value = false
  }
}

async function deleteProfile(id: string) {
  try {
    await invoke('delete_connection', { id })
    if (selectedProfileId.value === id) {
      selectedProfileId.value = '__new__'
      form.value = { name: '', endpoint: '', region: 'us-east-1', access_key_id: '', secret_access_key: '', path_style: true }
    }
    await loadProfiles()
    toast.success(t('auth.profiles.deleted'))
  } catch (e: any) {
    toast.error(typeof e === 'string' ? e : e.message)
  }
  deleteConfirmId.value = null
}

async function exportProfiles() {
  try {
    const profiles = await invoke<any[]>('export_profiles')
    const json = JSON.stringify(profiles, null, 2)
    const destPath = await save({
      defaultPath: 'client-s3-profiles.json',
      filters: [{ name: 'JSON', extensions: ['json'] }],
    })
    if (!destPath) return
    await writeTextFile(destPath, json)
    toast.success(t('auth.profiles.exported', { count: profiles.length }))
  } catch (e: any) {
    toast.error(typeof e === 'string' ? e : e.message)
  }
}

async function importProfiles() {
  try {
    const filePath = await openFileDialog({
      multiple: false,
      filters: [{ name: 'JSON', extensions: ['json'] }],
    })
    if (!filePath) return
    const path = Array.isArray(filePath) ? filePath[0] : filePath
    const content = await readTextFile(path)
    const profiles = JSON.parse(content)
    if (!Array.isArray(profiles)) {
      toast.error(t('auth.profiles.invalidFormat'))
      return
    }
    const imported = await invoke<ConnectionInfo[]>('import_profiles', { profiles })
    await loadProfiles()
    toast.success(t('auth.profiles.imported', { count: imported.length }))
  } catch (e: any) {
    toast.error(typeof e === 'string' ? e : e.message)
  }
}

function connectToProfile(id: string) {
  selectedProfileId.value = id
  activeTab.value = 'connect'
}
</script>

<template>
  <div class="w-full max-w-lg px-6">
    <div class="mb-6 flex flex-col items-center gap-3">
      <div class="flex size-14 items-center justify-center rounded-2xl bg-primary">
        <HardDrive class="size-7 text-primary-foreground" />
      </div>
      <div class="text-center">
        <h1 class="text-xl font-semibold tracking-tight">{{ t('auth.title') }}</h1>
        <p class="text-sm text-muted-foreground">
          {{ t('auth.subtitle') }}
        </p>
      </div>
    </div>

    <Tabs v-model:model-value="activeTab">
      <TabsList class="mb-4 w-full">
        <TabsTrigger value="connect" class="flex-1">{{ t('auth.tabs.connect') }}</TabsTrigger>
        <TabsTrigger value="profiles" class="flex-1">
          {{ t('auth.tabs.profiles') }}
          <Badge v-if="savedProfiles.length > 0" variant="secondary" class="ml-1.5 text-[10px]">
            {{ savedProfiles.length }}
          </Badge>
        </TabsTrigger>
      </TabsList>

      <!-- Connect tab -->
      <TabsContent value="connect">
        <Card>
          <CardHeader class="pb-4">
            <CardTitle class="text-base">{{ t('auth.connect.title') }}</CardTitle>
            <CardDescription class="text-xs">
              {{ t('auth.connect.description') }}
            </CardDescription>
          </CardHeader>
          <CardContent>
            <form class="grid gap-3.5" @submit.prevent="handleConnect">
              <!-- Profile quick-select -->
              <div v-if="savedProfiles.length > 0" class="grid gap-1.5">
                <Label class="text-xs">{{ t('auth.connect.savedProfile') }}</Label>
                <Select v-model:model-value="selectedProfileId">
                  <SelectTrigger>
                    <SelectValue :placeholder="t('auth.connect.choosePlaceholder')" />
                  </SelectTrigger>
                  <SelectContent>
                    <SelectItem value="__new__">{{ t('auth.connect.newConnection') }}</SelectItem>
                    <SelectItem
                      v-for="profile in savedProfiles"
                      :key="profile.id"
                      :value="profile.id"
                    >
                      {{ profile.name }} — {{ profile.endpoint }}
                    </SelectItem>
                  </SelectContent>
                </Select>
              </div>

              <Separator v-if="savedProfiles.length > 0" />

              <!-- Provider select -->
              <div v-if="selectedProfileId === '__new__'" class="grid gap-1.5">
                <Label class="text-xs">{{ t('auth.connect.provider') }}</Label>
                <div class="grid grid-cols-4 gap-1.5">
                  <button
                    v-for="provider in providers.slice(0, 8)"
                    :key="provider.id"
                    type="button"
                    class="flex flex-col items-center gap-1 overflow-hidden rounded-md border p-2 text-center text-[10px] transition-colors hover:bg-accent"
                    :class="selectedProvider === provider.id ? 'border-primary bg-accent' : 'border-border'"
                    @click="selectedProvider = provider.id"
                  >
                    <Cloud v-if="provider.icon === 'Cloud'" class="size-4 shrink-0" />
                    <HardDrive v-else class="size-4 shrink-0" />
                    <span class="w-full truncate leading-tight">{{ provider.name }}</span>
                  </button>
                </div>
                <a
                  v-if="currentProvider.helpUrl"
                  :href="currentProvider.helpUrl"
                  target="_blank"
                  class="flex items-center gap-1 text-[10px] text-muted-foreground hover:text-foreground"
                >
                  <ExternalLink class="size-3" />
                  {{ t('auth.connect.documentation', { name: currentProvider.name }) }}
                </a>
              </div>

              <!-- Form fields -->
              <div class="grid gap-1.5">
                <Label for="name" class="text-xs">{{ t('auth.connect.name') }}</Label>
                <Input id="name" v-model="form.name" :placeholder="t('auth.connect.namePlaceholder')" class="h-8 text-xs" required />
              </div>

              <div class="grid gap-1.5">
                <Label for="endpoint" class="text-xs">{{ t('auth.connect.endpoint') }}</Label>
                <Input
                  id="endpoint"
                  v-model="form.endpoint"
                  :placeholder="t('auth.connect.endpointPlaceholder')"
                  class="h-8 text-xs"
                  required
                />
              </div>

              <div class="grid gap-1.5">
                <Label for="region" class="text-xs">{{ t('auth.connect.region') }}</Label>
                <Select v-if="currentProvider.regions && selectedProfileId === '__new__'" v-model:model-value="form.region">
                  <SelectTrigger class="h-8 text-xs">
                    <SelectValue />
                  </SelectTrigger>
                  <SelectContent>
                    <SelectItem
                      v-for="r in currentProvider.regions"
                      :key="r.value"
                      :value="r.value"
                    >
                      {{ r.label }} ({{ r.value }})
                    </SelectItem>
                  </SelectContent>
                </Select>
                <Input v-else id="region" v-model:model-value="form.region" placeholder="us-east-1" class="h-8 text-xs" required />
              </div>

              <div class="grid gap-1.5">
                <Label for="access_key" class="text-xs">
                  {{ currentProvider.fields?.accessKeyLabel ?? t('auth.connect.accessKey') }}
                </Label>
                <Input
                  id="access_key"
                  v-model="form.access_key_id"
                  :placeholder="currentProvider.fields?.accessKeyPlaceholder ?? 'AKIAIOSFODNN7EXAMPLE'"
                  class="h-8 text-xs"
                  required
                />
              </div>

              <div class="grid gap-1.5">
                <Label for="secret_key" class="text-xs">
                  {{ currentProvider.fields?.secretKeyLabel ?? t('auth.connect.secretKey') }}
                </Label>
                <div class="relative">
                  <Input
                    id="secret_key"
                    v-model="form.secret_access_key"
                    :type="showSecret ? 'text' : 'password'"
                    :placeholder="selectedProfileId !== '__new__' ? t('auth.connect.secretReconnect') : '••••••••••••••••'"
                    class="h-8 pr-9 text-xs"
                    required
                  />
                  <button
                    type="button"
                    class="absolute right-2 top-1/2 -translate-y-1/2 text-muted-foreground hover:text-foreground"
                    @click="showSecret = !showSecret"
                  >
                    <EyeOff v-if="showSecret" class="size-3.5" />
                    <Eye v-else class="size-3.5" />
                  </button>
                </div>
              </div>

              <div class="flex items-center justify-between">
                <Label for="path_style" class="cursor-pointer text-xs">{{ t('auth.connect.pathStyle') }}</Label>
                <Switch id="path_style" v-model:model-value="form.path_style" />
              </div>

              <div v-if="selectedProfileId === '__new__'" class="flex items-center gap-2">
                <Checkbox id="save-profile" v-model:model-value="saveProfile" />
                <Label for="save-profile" class="cursor-pointer text-xs font-normal text-muted-foreground">
                  {{ t('auth.connect.remember') }}
                </Label>
              </div>

              <div
                v-if="error"
                class="rounded-md bg-destructive/10 px-3 py-2 text-xs text-destructive"
              >
                {{ error }}
              </div>

              <Button type="submit" class="h-9 w-full text-xs" :disabled="loading">
                <Spinner v-if="loading" class="mr-2 size-3.5" />
                <Plug v-else class="mr-2 size-3.5" />
                {{ loading ? t('auth.connect.connecting') : t('auth.connect.submit') }}
              </Button>
            </form>
          </CardContent>
        </Card>
      </TabsContent>

      <!-- Profiles tab -->
      <TabsContent value="profiles">
        <Card>
          <CardHeader class="pb-3">
            <div class="flex items-center justify-between">
              <div>
                <CardTitle class="text-base">{{ t('auth.profiles.title') }}</CardTitle>
                <CardDescription class="text-xs">
                  {{ t('auth.profiles.description') }}
                </CardDescription>
              </div>
              <div class="flex gap-1.5">
                <Button variant="outline" size="sm" class="h-7 text-xs" @click="importProfiles">
                  <Upload class="mr-1 size-3" />
                  {{ t('common.import') }}
                </Button>
                <Button
                  variant="outline"
                  size="sm"
                  class="h-7 text-xs"
                  :disabled="savedProfiles.length === 0"
                  @click="exportProfiles"
                >
                  <Download class="mr-1 size-3" />
                  {{ t('common.export') }}
                </Button>
              </div>
            </div>
          </CardHeader>
          <CardContent>
            <div v-if="savedProfiles.length === 0" class="py-8 text-center text-sm text-muted-foreground">
              {{ t('auth.profiles.empty') }}
              <br />
              <span class="text-xs">{{ t('auth.profiles.emptyHint') }}</span>
            </div>

            <ScrollArea v-else class="max-h-80">
              <div class="grid gap-2">
                <div
                  v-for="profile in savedProfiles"
                  :key="profile.id"
                  class="flex items-center justify-between rounded-md border px-3 py-2"
                >
                  <button
                    class="flex min-w-0 flex-1 flex-col text-left"
                    @click="connectToProfile(profile.id)"
                  >
                    <span class="truncate text-sm font-medium">{{ profile.name }}</span>
                    <span class="truncate text-[11px] text-muted-foreground">
                      {{ profile.endpoint }} · {{ profile.region }}
                    </span>
                  </button>
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
            </ScrollArea>
          </CardContent>
        </Card>
      </TabsContent>
    </Tabs>

    <!-- Delete confirm dialog -->
    <AlertDialog :open="deleteConfirmId !== null">
      <AlertDialogContent>
        <AlertDialogHeader>
          <AlertDialogTitle>{{ t('auth.profiles.deleteTitle') }}</AlertDialogTitle>
          <AlertDialogDescription>
            {{ t('auth.profiles.deleteDescription') }}
          </AlertDialogDescription>
        </AlertDialogHeader>
        <AlertDialogFooter>
          <AlertDialogCancel @click="deleteConfirmId = null">{{ t('common.cancel') }}</AlertDialogCancel>
          <AlertDialogAction class="bg-destructive hover:bg-destructive/90" @click="deleteProfile(deleteConfirmId!)">
            {{ t('common.delete') }}
          </AlertDialogAction>
        </AlertDialogFooter>
      </AlertDialogContent>
    </AlertDialog>
  </div>
</template>
