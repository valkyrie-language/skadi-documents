<script setup lang="ts">
import {useFluent} from 'fluent-vue'
import type {PackageDetail,} from "@/api/models";

defineProps<{
  packageInfo: PackageDetail
}>()

const {$t} = useFluent()

const formatDate = (date: string) => {
  return new Date(date).toLocaleDateString()
}
</script>

<template>
  <div class="package-header">
    <div class="header-content">
      <div class="title-section">
        <h1>
          <a :href="`/org/${packageInfo.org}`" class="org-link">{{ packageInfo.org }}</a>
          <span class="separator">/</span>
          <a :href="`/package/${packageInfo.org}/${packageInfo.name}`" class="name-link">{{ packageInfo.name }}</a>
          <span class="version">v{{ packageInfo.version }}</span>
        </h1>
      </div>
      <p class="description">{{ packageInfo.description }}</p>

      <div class="meta-info">
        <span class="meta-item">
          <span class="meta-label">{{ $t('package-author') }}</span>
          <span class="meta-value">{{ packageInfo.author }}</span>
        </span>
        <span class="meta-item">
          <span class="meta-label">{{ $t('package-license') }}</span>
          <span class="meta-value">{{ packageInfo.license }}</span>
        </span>
        <span class="meta-item">
          <span class="meta-label">{{ $t('package-repository') }}</span>
          <span class="meta-value">
            <a :href="packageInfo.repository" target="_blank" rel="noopener" class="repository-link">
              <span class="repo-icon">📦</span>
              <span class="repo-text">{{ packageInfo.repository.replace(/^https?:\/\/(www\.)?/, '') }}</span>
            </a>
          </span>
        </span>
        <span class="meta-item">
          <span class="meta-label">{{ $t('package-downloads') }}</span>
          <span class="meta-value">{{ packageInfo.downloads }}</span>
        </span>
        <span class="meta-item">
          <span class="meta-label">{{ $t('package-last-update') }}</span>
          <span class="meta-value">{{ formatDate(packageInfo.last_update) }}</span>
        </span>
      </div>
    </div>
  </div>
</template>

<style lang="scss" scoped>
.package-header {
  background-color: #fff;
  border-bottom: 1px solid #e5e7eb;
  margin-bottom: 2rem;
  padding: 2rem 0;

  .header-content {
    max-width: 1200px;
    margin: 0 auto;
    padding: 0 1rem;
  }

  .title-section {
    h1 {
      margin: 0;
      font-size: 2.5rem;
      color: #000;
      font-weight: 600;
      display: flex;
      align-items: center;
      gap: 0.25rem;

      .org-link,
      .name-link {
        color: inherit;
        text-decoration: none;
        transition: color 0.2s;

        &:hover {
          color: #4f46e5;
        }
      }

      .separator {
        color: #6b7280;
        margin: 0 0.25rem;
      }

      .version {
        font-size: 1.5rem;
        color: #6b7280;
        font-weight: 400;
      }
    }
  }

  .description {
    margin: 1rem 0 1.5rem;
    color: #374151;
    font-size: 1.125rem;
    line-height: 1.5;
    max-width: 800px;
  }

  .meta-info {
    display: flex;
    flex-wrap: wrap;
    gap: 1.5rem;

    .meta-item {
      display: flex;
      align-items: center;
      gap: 0.5rem;

      .meta-label {
        color: #6b7280;
        font-size: 0.875rem;
      }

      .meta-value {
        color: #111827;
        font-size: 0.875rem;
        font-weight: 500;

        .repository-link {
          display: inline-flex;
          align-items: center;
          gap: 0.5rem;
          color: #4f46e5;
          text-decoration: none;
          padding: 0.25rem 0.5rem;
          border-radius: 0.375rem;
          transition: background-color 0.2s;

          &:hover {
            background-color: #f3f4f6;
          }

          .repo-icon {
            font-size: 1rem;
          }

          .repo-text {
            font-weight: 500;
          }
        }
      }
    }
  }
}

</style>