<script setup lang="ts">
import { onMounted } from 'vue'
import { useBrowserStore } from '@/stores/browser'
import { useI18n } from 'vue-i18n'
import {
  SidebarGroup,
  SidebarGroupLabel,
  SidebarGroupAction,
  SidebarGroupContent,
  SidebarMenu,
  SidebarMenuButton,
  SidebarMenuItem,
} from '@/components/ui/sidebar'
import {
  ContextMenu,
  ContextMenuContent,
  ContextMenuItem,
  ContextMenuTrigger,
  ContextMenuSeparator,
} from '@/components/ui/context-menu'
import { Database, Plus, Trash2 } from 'lucide-vue-next'

const { t } = useI18n()

const emit = defineEmits<{
  createBucket: []
}>()

const store = useBrowserStore()

onMounted(() => {
  store.loadBuckets()
})
</script>

<template>
  <SidebarGroup>
    <SidebarGroupLabel>
      {{ t('browser.buckets') }}
      <SidebarGroupAction @click="emit('createBucket')">
        <Plus class="size-4" />
      </SidebarGroupAction>
    </SidebarGroupLabel>
    <SidebarGroupContent>
      <SidebarMenu>
        <SidebarMenuItem v-for="bucket in store.buckets" :key="bucket.name">
          <ContextMenu>
            <ContextMenuTrigger as-child>
              <SidebarMenuButton
                :is-active="store.currentBucket === bucket.name"
                @click="store.navigateToBucket(bucket.name)"
              >
                <Database class="size-4" />
                <span>{{ bucket.name }}</span>
              </SidebarMenuButton>
            </ContextMenuTrigger>
            <ContextMenuContent>
              <ContextMenuItem @click="store.navigateToBucket(bucket.name)">
                <Database class="mr-2 size-4" />
                {{ t('common.open') }}
              </ContextMenuItem>
              <ContextMenuSeparator />
              <ContextMenuItem
                class="text-destructive"
                @click="store.removeBucket(bucket.name)"
              >
                <Trash2 class="mr-2 size-4" />
                {{ t('browser.deleteBucket') }}
              </ContextMenuItem>
            </ContextMenuContent>
          </ContextMenu>
        </SidebarMenuItem>

        <SidebarMenuItem v-if="store.buckets.length === 0 && !store.loading">
          <SidebarMenuButton class="text-muted-foreground" @click="emit('createBucket')">
            <Plus class="size-4" />
            <span>{{ t('browser.createBucket') }}</span>
          </SidebarMenuButton>
        </SidebarMenuItem>
      </SidebarMenu>
    </SidebarGroupContent>
  </SidebarGroup>
</template>
