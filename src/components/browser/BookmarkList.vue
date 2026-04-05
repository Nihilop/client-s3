<script setup lang="ts">
import { ref } from 'vue'
import { useBookmarkStore } from '@/stores/bookmarks'
import { useBrowserStore } from '@/stores/browser'
import { useI18n } from 'vue-i18n'
import {
  SidebarGroup,
  SidebarGroupLabel,
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
} from '@/components/ui/context-menu'
import { Input } from '@/components/ui/input'
import { Star, Trash2, Plus } from 'lucide-vue-next'
import { toast } from 'vue-sonner'

const { t } = useI18n()

const bookmarkStore = useBookmarkStore()
const browserStore = useBrowserStore()

const isAdding = ref(false)
const newName = ref('')

function navigateToBookmark(bucket: string, prefix: string) {
  browserStore.navigateToBucket(bucket)
  if (prefix) {
    // Need to wait for bucket navigation, then set prefix
    setTimeout(() => {
      browserStore.navigateToPrefix(prefix)
    }, 100)
  }
}

function startAdding() {
  // Default name: last folder segment or bucket name
  const prefix = browserStore.currentPrefix
  if (prefix) {
    const parts = prefix.split('/').filter(Boolean)
    newName.value = parts[parts.length - 1] ?? ''
  } else {
    newName.value = browserStore.currentBucket ?? ''
  }
  isAdding.value = true
}

function confirmAdd() {
  if (!newName.value.trim() || !browserStore.currentBucket) return
  bookmarkStore.addBookmark(
    newName.value.trim(),
    browserStore.currentBucket,
    browserStore.currentPrefix,
  )
  toast.success(t('bookmarks.added'))
  isAdding.value = false
  newName.value = ''
}

function cancelAdd() {
  isAdding.value = false
  newName.value = ''
}

function removeBookmark(id: string) {
  bookmarkStore.removeBookmark(id)
  toast.success(t('bookmarks.removed'))
}
</script>

<template>
  <SidebarGroup>
    <SidebarGroupLabel>
      {{ t('bookmarks.title') }}
    </SidebarGroupLabel>
    <SidebarGroupContent>
      <SidebarMenu>
        <SidebarMenuItem v-for="bookmark in bookmarkStore.bookmarks" :key="bookmark.id">
          <ContextMenu>
            <ContextMenuTrigger as-child>
              <SidebarMenuButton
                @click="navigateToBookmark(bookmark.bucket, bookmark.prefix)"
              >
                <Star class="size-4" />
                <span>{{ bookmark.name }}</span>
              </SidebarMenuButton>
            </ContextMenuTrigger>
            <ContextMenuContent>
              <ContextMenuItem
                class="text-destructive"
                @click="removeBookmark(bookmark.id)"
              >
                <Trash2 class="mr-2 size-4" />
                {{ t('bookmarks.remove') }}
              </ContextMenuItem>
            </ContextMenuContent>
          </ContextMenu>
        </SidebarMenuItem>

        <!-- Empty state -->
        <SidebarMenuItem v-if="bookmarkStore.bookmarks.length === 0">
          <div class="px-2 py-1.5 text-xs text-muted-foreground">
            {{ t('bookmarks.empty') }}
            <br />
            <span class="text-[10px]">{{ t('bookmarks.emptyHint') }}</span>
          </div>
        </SidebarMenuItem>

        <!-- Add current location -->
        <SidebarMenuItem v-if="browserStore.currentBucket && !isAdding">
          <SidebarMenuButton class="text-muted-foreground" @click="startAdding">
            <Plus class="size-4" />
            <span>{{ t('bookmarks.add') }}</span>
          </SidebarMenuButton>
        </SidebarMenuItem>

        <!-- Inline add form -->
        <SidebarMenuItem v-if="isAdding">
          <div class="px-2 py-1">
            <Input
              v-model="newName"
              :placeholder="t('bookmarks.namePrompt')"
              class="h-7 text-xs"
              autofocus
              @keydown.enter="confirmAdd"
              @keydown.escape="cancelAdd"
            />
          </div>
        </SidebarMenuItem>
      </SidebarMenu>
    </SidebarGroupContent>
  </SidebarGroup>
</template>
