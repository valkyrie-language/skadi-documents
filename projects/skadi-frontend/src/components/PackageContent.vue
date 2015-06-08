<script setup lang="ts">
import { useFluent } from 'fluent-vue'

defineProps<{
  activeTab: string
  packageInfo: {
    name: string
    description: string
  }
}>()

const { $t } = useFluent()
</script>

<template>
  <div class="content-area">
    <!-- 根据activeTab显示不同内容 -->
    <div v-if="activeTab === 'overview'" class="overview-content">
      <h2>Overview</h2>
      <p>{{ packageInfo.description }}</p>
    </div>
    <div v-else-if="activeTab === 'docs'" class="docs-content">
      <h2>Documentation</h2>
      <!-- 文档内容 -->
    </div>
    <div v-else-if="activeTab === 'files'" class="files-content">
      <h2>Files</h2>
      <!-- 文件列表 -->
    </div>
    <div v-else-if="activeTab === 'dependencies'" class="dependencies-content">
      <h2>Dependencies</h2>
      <!-- 依赖列表 -->
    </div>
    <div v-else-if="activeTab === 'dependents'" class="dependents-content">
      <h2>Dependents</h2>
      <!-- 被依赖列表 -->
    </div>
  </div>
</template>

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