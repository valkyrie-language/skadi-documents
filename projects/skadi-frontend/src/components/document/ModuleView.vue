<template>
  <div class="module-view">
    <div class="module-header">
      <h1>{{ moduleName }}</h1>
      <div class="module-meta">
        <span class="version">{{ version }}</span>
      </div>
    </div>
    <div class="module-content">
      <div class="doc-section">
        <h2>Description</h2>
        <div class="description" v-if="description">
          {{ description }}
        </div>
        <div class="no-description" v-else>
          No description available.
        </div>
      </div>

      <div class="doc-section">
        <div class="items-list">
          <div v-for="(items, type) in groupedItems" :key="type" class="item-group">
            <h3 class="group-title">{{ type }}</h3>
            <div v-for="item in items" :key="item.name" class="item">
              <div class="item-header">
                <span class="item-type" :class="`type-${item.type.toLowerCase()}`">{{ item.type }}</span>
                <span class="item-name">{{ item.name }}</span>
              </div>
              <div class="item-description" v-if="item.summary">
                {{ item.summary }}
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import {computed, ref} from 'vue'
import {useRoute} from 'vue-router'
import {type DocumentInfo, type ModuleItem, ModuleType} from "@/api/models";

const route = useRoute()
const moduleName = ref(route.params.module as string)
const version = ref(route.params.version as string)

const ITEM_ORDER: ModuleType[] = ['module', 'class', 'trait', 'constant']

const props = defineProps<{
  moduleInfo: DocumentInfo
}>()

const items = computed(() => {
  return props.moduleInfo.items as ModuleItem[]
})

const groupedItems = computed(() => {
  const groups: Record<ModuleType, ModuleItem[]> = {}
  for (const type of ITEM_ORDER) {
    for (const item of items.value) {
      if (item.type === type) {
        if (!groups[type]) {
          groups[type] = []
        }
        groups[type].push(item)
      }
    }
  }
  return groups
})
const description = computed(() => {
  return props.moduleInfo.documentation
})
</script>

<style lang="scss" scoped>
.module-view {
  padding: 2rem;

  .module-header {
    margin-bottom: 2rem;

    h1 {
      font-size: 2rem;
      margin-bottom: 0.5rem;
    }

    .module-meta {
      color: #666;

      .version {
        background: #f0f0f0;
        padding: 0.2rem 0.5rem;
        border-radius: 4px;
        font-size: 0.9rem;
      }
    }
  }

  .doc-section {
    margin-bottom: 2rem;

    h2 {
      font-size: 1.5rem;
      margin-bottom: 1rem;
      color: #333;
    }
  }

  .items-list {
    .item-group {
      margin-bottom: 2rem;

      .group-title {
        font-size: 1.2rem;
        color: #333;
        margin-bottom: 1rem;
        text-align: left;
        border-bottom: 2px solid #eee;
        padding-bottom: 0.5rem;
      }

      .item {
        padding: 1rem;
        border: 1px solid #eee;
        border-radius: 4px;
        margin-bottom: 1rem;

        &:hover {
          background: #f8f9fa;
        }

        .item-header {
          display: flex;
          align-items: center;
          gap: 1rem;
          margin-bottom: 0.5rem;

          .item-type {
            padding: 0.2rem 0.5rem;
            border-radius: 4px;
            font-size: 0.9rem;
            color: #fff;

            &.type-module {
              background: #007bff;
            }

            &.type-class {
              background: #28a745;
            }

            &.type-trait {
              background: #6610f2;
            }
          }

          .item-name {
            font-weight: 500;
            color: #333;
          }
        }

        .item-description {
          color: #666;
          font-size: 0.9rem;
        }
      }
    }
  }
}
</style>