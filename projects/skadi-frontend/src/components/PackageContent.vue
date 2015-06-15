<script setup lang="ts">
import { useFluent } from 'fluent-vue'
import { ref } from 'vue'

defineProps<{
  activeTab: string
  packageInfo: {
    name: string
    description: string
  }
}>()

const { $t } = useFluent()
const tabs = ref([
  { id: 'overview', label: 'package-overview' },
  { id: 'docs', label: 'package-docs' },
  { id: 'files', label: 'package-files' },
  { id: 'dependencies', label: 'package-dependencies' },
  { id: 'dependents', label: 'package-dependents' }
])
</script>

<template>
  <div class="package-content">
    <div class="tabs">
      <button
        v-for="tab in tabs"
        :key="tab.id"
        :class="['tab-button', { active: activeTab === tab.id }]"
      >
        {{ $t(tab.label) }}
      </button>
    </div>

    <div class="content-area">
      <div v-if="activeTab === 'overview'" class="overview-content">
        <h2>{{ $t('package-overview') }}</h2>
        <p>{{ packageInfo.description }}</p>
      </div>
      <div v-else-if="activeTab === 'docs'" class="docs-content">
        <h2>{{ $t('package-docs') }}</h2>
        <!-- 文档内容 -->
      </div>
      <div v-else-if="activeTab === 'files'" class="files-content">
        <h2>{{ $t('package-files') }}</h2>
        <!-- 文件列表 -->
      </div>
      <div v-else-if="activeTab === 'dependencies'" class="dependencies-content">
        <h2>{{ $t('package-dependencies') }}</h2>
        <!-- 依赖列表 -->
      </div>
      <div v-else-if="activeTab === 'dependents'" class="dependents-content">
        <h2>{{ $t('package-dependents') }}</h2>
        <!-- 被依赖列表 -->
      </div>
    </div>
  </div>
</template>

<style lang="scss" scoped>
.package-content {
  .tabs {
    display: flex;
    gap: 1rem;
    padding: 0 2rem;
    border-bottom: 1px solid #eee;

    .tab-button {
      padding: 1rem;
      border: none;
      background: none;
      color: #666;
      font-size: 1rem;
      cursor: pointer;
      border-bottom: 2px solid transparent;
      transition: all 0.2s;

      &:hover {
        color: #333;
      }

      &.active {
        color: #0969da;
        border-bottom-color: #0969da;
      }
    }
  }

  .content-area {
    padding: 2rem;

    h2 {
      margin: 0 0 1.5rem;
      font-size: 1.5rem;
      color: #333;
    }

    p {
      color: #666;
      line-height: 1.6;
    }
  }
}
</style>

<style lang="scss" scoped>
.content-area {
  background-color: white;
  border-radius: 8px;
  padding: 2rem;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);

  h2 {
    font-size: 1.5rem;
    color: #212529;
    margin: 0 0 1rem;
  }
}
</style>