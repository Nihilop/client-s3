<script setup lang="ts">
import { useBrowserStore } from '@/stores/browser'
import {
  Breadcrumb,
  BreadcrumbItem,
  BreadcrumbLink,
  BreadcrumbList,
  BreadcrumbPage,
  BreadcrumbSeparator,
} from '@/components/ui/breadcrumb'
import { Database } from 'lucide-vue-next'

const store = useBrowserStore()
</script>

<template>
  <Breadcrumb>
    <BreadcrumbList>
      <BreadcrumbItem v-if="store.currentBucket">
        <BreadcrumbLink
          class="flex cursor-pointer items-center gap-1"
          @click="store.navigateToBucket(store.currentBucket!)"
        >
          <Database class="size-3" />
          {{ store.currentBucket }}
        </BreadcrumbLink>
      </BreadcrumbItem>

      <template v-for="(crumb, i) in store.breadcrumbs" :key="crumb.prefix">
        <BreadcrumbSeparator />
        <BreadcrumbItem>
          <BreadcrumbPage v-if="i === store.breadcrumbs.length - 1">
            {{ crumb.name }}
          </BreadcrumbPage>
          <BreadcrumbLink
            v-else
            class="cursor-pointer"
            @click="store.navigateToPrefix(crumb.prefix)"
          >
            {{ crumb.name }}
          </BreadcrumbLink>
        </BreadcrumbItem>
      </template>
    </BreadcrumbList>
  </Breadcrumb>
</template>
