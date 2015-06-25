<template>
  <div class="version-item" :class="{ 'version-item--latest': isLatest }">
    <div class="version-info">
      <div class="version-header">
        <h3 class="version-number">{{ version.version }}</h3>
        <span v-if="isLatest" class="latest-tag">{{ $t('latest') }}</span>
      </div>
      <div class="version-meta">
        <span class="meta-item">
          <i class="fas fa-calendar"></i>
          {{ formatDate(version.publishedAt) }}
        </span>
        <span class="meta-item">
          <i class="fas fa-download"></i>
          {{ formatDownloads(version.downloads) }}
        </span>
        <span class="meta-item">
          <i class="fas fa-file-archive"></i>
          {{ formatSize(version.size) }}
        </span>
      </div>
    </div>
    <div class="version-author">
      <img :src="version.author.avatar" :alt="version.author.name" class="author-avatar">
      <span class="author-name">{{ version.author.name }}</span>
    </div>
    <div class="version-license" v-if="version.license">
      <i class="fas fa-balance-scale"></i>
      {{ version.license }}
    </div>
  </div>
</template>

<script setup lang="ts">
import { useFluent } from 'fluent-vue'
import { computed } from 'vue'
import type { PackageVersion } from '@/api/models'

interface Props {
  version: PackageVersion
  isLatest?: boolean
}

const props = withDefaults(defineProps<Props>(), {
  isLatest: false
})

const { $t } = useFluent()

const formatDate = (date: string) => {
  return new Date(date).toLocaleDateString()
}

const formatDownloads = (downloads: number) => {
  if (downloads >= 1000000) {
    return `${(downloads / 1000000).toFixed(1)}M`
  }
  if (downloads >= 1000) {
    return `${(downloads / 1000).toFixed(1)}K`
  }
  return downloads.toString()
}

const formatSize = (size: number) => {
  const units = ['B', 'KB', 'MB', 'GB']
  let value = size
  let unitIndex = 0

  while (value >= 1024 && unitIndex < units.length - 1) {
    value /= 1024
    unitIndex++
  }

  return `${value.toFixed(1)} ${units[unitIndex]}`
}
</script>

<style lang="scss" scoped>
.version-item {
  padding: 1.5rem;
  border: 1px solid #e5e7eb;
  border-radius: 8px;
  margin-bottom: 1rem;
  transition: all 0.2s ease;

  &:hover {
    border-color: #4f46e5;
    box-shadow: 0 2px 4px rgba(79, 70, 229, 0.1);
  }

  &--latest {
    border-color: #4f46e5;
    background-color: rgba(79, 70, 229, 0.02);
  }
}

.version-header {
  display: flex;
  align-items: center;
  gap: 0.75rem;
  margin-bottom: 0.5rem;
}

.version-number {
  margin: 0;
  font-size: 1.25rem;
  color: #111827;
}

.latest-tag {
  background-color: #4f46e5;
  color: white;
  padding: 0.25rem 0.5rem;
  border-radius: 4px;
  font-size: 0.875rem;
}

.version-meta {
  display: flex;
  gap: 1.5rem;
  color: #6b7280;
  font-size: 0.875rem;
}

.meta-item {
  display: flex;
  align-items: center;
  gap: 0.375rem;

  i {
    font-size: 1rem;
  }
}

.version-author {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  margin-top: 1rem;
}

.author-avatar {
  width: 24px;
  height: 24px;
  border-radius: 50%;
}

.author-name {
  color: #4f46e5;
  font-size: 0.875rem;
}

.version-license {
  margin-top: 0.5rem;
  color: #6b7280;
  font-size: 0.875rem;
  display: flex;
  align-items: center;
  gap: 0.375rem;
}
</style>