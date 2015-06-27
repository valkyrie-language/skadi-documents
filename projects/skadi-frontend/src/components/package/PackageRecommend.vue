<script setup lang="ts">
import {useFluent} from 'fluent-vue'

defineProps<{
  title: string
  packages: Array<{
    name: string
    version: string
    description: string
  }>
}>()

const {$t} = useFluent()
</script>

<template>
  <section class="package-section">
    <h2>{{ $t(title) }}</h2>
    <div class="package-cards">
      <div class="package-card" v-for="pkg in packages" :key="pkg.name">
        <h3>{{ pkg.name }}</h3>
        <p class="version">{{ $t('version', {version: pkg.version}) }}</p>
        <p class="description">{{ $t('package-description', {description: pkg.description}) }}</p>
      </div>
    </div>
    <div class="view-more">
      <button class="view-more-button">{{ $t('view-more') }}</button>
    </div>
  </section>
</template>

<style lang="scss" scoped>
.package-section {
  background-color: white;
  border-radius: 8px;
  padding: 1.5rem;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
  height: 100%;
  display: flex;
  flex-direction: column;

  h2 {
    margin: 0 0 1rem;
    color: #343a40;
    font-size: 1.5rem;
  }
}

.package-cards {
  flex: 1;
  display: flex;
  flex-direction: column;
  gap: 1rem;
}

.package-card {
  padding: 1rem;
  border: 1px solid #e9ecef;
  border-radius: 4px;
  transition: all 0.2s;

  &:hover {
    border-color: #4dabf7;
    box-shadow: 0 2px 4px rgba(77, 171, 247, 0.1);
  }

  h3 {
    margin: 0;
    color: #4dabf7;
    font-size: 1.1rem;
  }

  .version {
    margin: 0.5rem 0;
    color: #868e96;
    font-size: 0.9rem;
  }

  .description {
    margin: 0;
    color: #495057;
    font-size: 0.95rem;
    line-height: 1.5;
  }
}

.view-more {
  margin-top: 1rem;
  text-align: center;

  .view-more-button {
    padding: 0.5rem 1rem;
    background-color: transparent;
    border: 2px solid #4dabf7;
    color: #4dabf7;
    border-radius: 4px;
    cursor: pointer;
    transition: all 0.2s;

    &:hover {
      background-color: #4dabf7;
      color: white;
    }
  }
}
</style>