<script setup lang="ts">
import { useFluent } from 'fluent-vue'

defineProps<{
  packageInfo: {
    name: string
    version: string
    description: string
    author: string
    license: string
    repository: string
    downloads: string
    lastUpdate: string
  }
}>()

const { $t } = useFluent()
</script>

<template>
  <div class="package-header">
    <h1>{{ packageInfo.name }}</h1>
    <p class="description">{{ packageInfo.description }}</p>

    <div class="meta-info">
      <div class="meta-item version">
        <span class="label">{{ $t('package-version') }}</span>
        <span class="value">{{ packageInfo.version }}</span>
      </div>
      <div class="meta-item author">
        <span class="label">{{ $t('package-author') }}</span>
        <span class="value">{{ packageInfo.author }}</span>
      </div>
      <div class="meta-item license">
        <span class="label">{{ $t('package-license') }}</span>
        <span class="value">{{ packageInfo.license }}</span>
      </div>
      <div class="meta-item repository">
        <span class="label">{{ $t('package-repository') }}</span>
        <span class="value">
          <a :href="packageInfo.repository" target="_blank" rel="noopener">{{ packageInfo.repository }}</a>
        </span>
      </div>
      <div class="meta-item downloads">
        <span class="label">{{ $t('package-downloads') }}</span>
        <span class="value">{{ packageInfo.downloads }}</span>
      </div>
      <div class="meta-item last-update">
        <span class="label">{{ $t('package-last-update') }}</span>
        <span class="value">{{ packageInfo.lastUpdate }}</span>
      </div>
    </div>
  </div>
</template>

<style lang="scss" scoped>
.package-header {
  padding: 2rem;
  border-bottom: 1px solid #eee;

  h1 {
    margin: 0;
    font-size: 2rem;
    color: #333;
  }

  .description {
    margin: 1rem 0;
    color: #666;
    font-size: 1.1rem;
  }

  .meta-info {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
    gap: 1rem;
    margin-top: 2rem;

    .meta-item {
      display: flex;
      flex-direction: column;
      padding: 1rem;
      background: #f8f9fa;
      border-radius: 8px;
      transition: background-color 0.2s;

      &:hover {
        background: #f1f3f5;
      }

      .label {
        color: #666;
        font-size: 0.9rem;
        margin-bottom: 0.5rem;
      }

      .value {
        color: #333;
        font-weight: 500;

        a {
          color: #0969da;
          text-decoration: none;

          &:hover {
            text-decoration: underline;
          }
        }
      }
    }
  }
}
</style>

<style lang="scss" scoped>
.package-header {
  background-color: white;
  border-radius: 8px;
  padding: 2rem;
  margin-bottom: 2rem;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);

  h1 {
    font-size: 2rem;
    color: #212529;
    margin: 0 0 1rem;
  }

  .description {
    font-size: 1.1rem;
    color: #495057;
    margin-bottom: 1.5rem;
  }

  .meta-info {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
    gap: 1rem;

    .meta-item {
      .label {
        display: block;
        font-size: 0.875rem;
        color: #868e96;
        margin-bottom: 0.25rem;
      }

      .value {
        font-size: 1rem;
        color: #212529;
        font-weight: 500;
      }
    }
  }
}
</style>