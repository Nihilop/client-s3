<script setup lang="ts">
import { ref } from 'vue'
import { useI18n } from 'vue-i18n'
import { useTransfers, type TransferStatus } from '@/composables/useTransfers'
import { Button } from '@/components/ui/button'
import { Progress } from '@/components/ui/progress'
import { ScrollArea } from '@/components/ui/scroll-area'
import {
  ChevronUp,
  ChevronDown,
  Check,
  X,
  Loader2,
  Upload,
} from 'lucide-vue-next'

const { t } = useI18n()
const { transfers, activeCount, clearCompleted } = useTransfers()

const collapsed = ref(false)

function statusLabel(status: TransferStatus): string {
  if (status === 'Completed') return t('transfers.completed')
  if (status === 'Cancelled') return t('transfers.cancelled')
  if (typeof status === 'object' && 'Failed' in status) return t('transfers.failed')
  return ''
}

function failedMessage(status: TransferStatus): string {
  if (typeof status === 'object' && 'Failed' in status) return status.Failed
  return ''
}
</script>

<template>
  <div
    v-if="transfers.length > 0"
    class="shrink-0 border-t border-border bg-background"
  >
    <!-- Header bar -->
    <div
      class="flex h-8 items-center gap-2 px-3 cursor-pointer select-none"
      @click="collapsed = !collapsed"
    >
      <Upload class="size-3.5 text-muted-foreground" />
      <span class="text-xs font-medium">
        {{ t('transfers.title') }}
      </span>
      <span
        v-if="activeCount > 0"
        class="text-xs text-muted-foreground"
      >
        {{ t('transfers.active', { count: activeCount }) }}
      </span>

      <div class="ml-auto flex items-center gap-1">
        <Button
          variant="ghost"
          size="icon"
          class="size-6"
          @click.stop="clearCompleted"
        >
          <span class="text-[10px]">{{ t('transfers.clear') }}</span>
        </Button>
        <component
          :is="collapsed ? ChevronUp : ChevronDown"
          class="size-3.5 text-muted-foreground"
        />
      </div>
    </div>

    <!-- Transfer list -->
    <Transition
      enter-active-class="transition-[max-height,opacity] duration-200 ease-out"
      leave-active-class="transition-[max-height,opacity] duration-200 ease-in"
      enter-from-class="max-h-0 opacity-0"
      enter-to-class="max-h-[200px] opacity-100"
      leave-from-class="max-h-[200px] opacity-100"
      leave-to-class="max-h-0 opacity-0"
    >
      <div v-if="!collapsed" class="overflow-hidden">
        <ScrollArea class="max-h-[200px]">
          <div class="space-y-1 px-3 pb-2">
            <div
              v-for="transfer in transfers"
              :key="transfer.id"
              class="flex items-center gap-2 rounded-md px-2 py-1.5 text-xs"
            >
              <!-- Status icon -->
              <Loader2
                v-if="transfer.status === 'InProgress' || transfer.status === 'Pending'"
                class="size-3.5 shrink-0 animate-spin text-primary"
              />
              <Check
                v-else-if="transfer.status === 'Completed'"
                class="size-3.5 shrink-0 text-green-500"
              />
              <X
                v-else
                class="size-3.5 shrink-0 text-destructive"
              />

              <!-- File name -->
              <span class="min-w-0 flex-1 truncate">
                {{ transfer.fileName }}
              </span>

              <!-- Status label for terminal states -->
              <span
                v-if="transfer.status === 'Completed' || transfer.status === 'Cancelled' || (typeof transfer.status === 'object' && 'Failed' in transfer.status)"
                class="shrink-0 text-[10px] text-muted-foreground"
                :title="failedMessage(transfer.status)"
              >
                {{ statusLabel(transfer.status) }}
              </span>

              <!-- Progress bar and percentage for active transfers -->
              <template v-if="transfer.status === 'InProgress' || transfer.status === 'Pending'">
                <Progress
                  :model-value="transfer.progress"
                  class="h-1.5 w-20 shrink-0"
                />
                <span class="w-8 shrink-0 text-right text-[10px] text-muted-foreground">
                  {{ transfer.progress }}%
                </span>
              </template>
            </div>
          </div>
        </ScrollArea>
      </div>
    </Transition>
  </div>
</template>
