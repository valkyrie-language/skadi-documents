<template>
  <div class="versions-content">
    <div class="versions-header">
      <h2>{{ $t('package-versions') }}</h2>
      <div class="sort-controls">
        <button
          v-for="option in sortOptions"
          :key="option.key"
          class="sort-button"
          :class="{ active: currentSort === option.key }"
          @click="toggleSort(option.key)"
        >
          {{ $t(option.label) }}
          <i class="fas" :class="getSortIcon(option.key)"></i>
        </button>
      </div>
    </div>
    <div class="versions-list">
      <PackageVersionItem
        v-for="version in sortedVersions"
        :key="version.version"
        :version="version"
        :is-latest="version === sortedVersions[0]"
      />
    </div>
  </div>
</template>

<script setup lang="ts">
import { useFluent } from 'fluent-vue'
import { ref, computed } from 'vue'
import type { PackageInfo, PackageVersion } from '@/api/models'
import PackageVersionItem from './PackageVersionItem.vue'

defineProps<{
  packageInfo: PackageInfo
}>()

const { $t } = useFluent()
</script>

<style lang="scss" scoped>
.versions-content {
  padding: 1rem;

  .versions-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 1.5rem;

    h2 {
      margin: 0;
      font-size: 1.5rem;
      color: #333;
    }
  }

  .sort-controls {
    display: flex;
    gap: 0.5rem;
  }

  .sort-button {
    display: flex;
    align-items: center;
    gap: 0.375rem;
    padding: 0.5rem 0.75rem;
    border: 1px solid #e5e7eb;
    border-radius: 6px;
    background-color: white;
    color: #6b7280;
    font-size: 0.875rem;
    cursor: pointer;
    transition: all 0.2s ease;

    &:hover {
      border-color: #4f46e5;
      color: #4f46e5;
    }

    &.active {
      border-color: #4f46e5;
      color: #4f46e5;
      background-color: rgba(79, 70, 229, 0.05);
    }

    i {
      font-size: 0.875rem;
    }
  }

  .versions-list {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }
}
</style>