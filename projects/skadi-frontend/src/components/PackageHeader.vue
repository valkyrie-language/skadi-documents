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
        <span class="label">Version</span>
        <span class="value">{{ packageInfo.version }}</span>
      </div>
      <div class="meta-item license">
        <span class="label">License</span>
        <span class="value">{{ packageInfo.license }}</span>
      </div>
      <div class="meta-item downloads">
        <span class="label">Downloads</span>
        <span class="value">{{ packageInfo.downloads }}</span>
      </div>
      <div class="meta-item last-update">
        <span class="label">Last Update</span>
        <span class="value">{{ packageInfo.lastUpdate }}</span>
      </div>
    </div>
  </div>
</template>

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