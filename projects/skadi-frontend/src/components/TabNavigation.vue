<script setup lang="ts">
import { useFluent } from 'fluent-vue'

defineProps<{
  tabs: Array<{
    id: string
    label: string
  }>
  activeTab: string
}>();

const emit = defineEmits<{
  'update:activeTab': [value: string]
}>();

const { $t } = useFluent();
</script>

<template>
  <nav class="tabs">
    <button
      v-for="tab in tabs"
      :key="tab.id"
      :class="['tab-button', { active: activeTab === tab.id }]"
      @click="emit('update:activeTab', tab.id)"
    >
      {{ tab.label }}
    </button>
  </nav>
</template>

<style lang="scss" scoped>
.tabs {
  display: flex;
  gap: 1rem;
  margin-bottom: 2rem;
  border-bottom: 2px solid #e9ecef;
  padding-bottom: 1rem;

  .tab-button {
    padding: 0.5rem 1rem;
    background: none;
    border: none;
    color: #495057;
    font-size: 1rem;
    cursor: pointer;
    transition: all 0.2s;

    &:hover {
      color: #212529;
    }

    &.active {
      color: #4dabf7;
      font-weight: 500;
      position: relative;

      &::after {
        content: '';
        position: absolute;
        bottom: -1rem;
        left: 0;
        width: 100%;
        height: 2px;
        background-color: #4dabf7;
      }
    }
  }
}
</style>