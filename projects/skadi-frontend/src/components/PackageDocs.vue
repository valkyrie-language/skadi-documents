<template>
  <div class="docs-content">
    <h2>{{ $t('package-docs') }}</h2>
    <!-- 文档内容将在此处展示 -->
    <div class="docs-placeholder">
      <p>{{ $t('docs-coming-soon') }}</p>
    </div>
  </div>
</template>

<script setup lang="ts">
import {useFluent} from 'fluent-vue'
import type {PackageInfo} from '@/api/models'

defineProps<{
  packageInfo: PackageInfo
}>()

const {$t} = useFluent()
</script>

<style lang="scss" scoped>
.docs-content {
  padding: 1rem;

  h2 {
    margin-bottom: 1rem;
    font-size: 1.5rem;
    color: #333;
  }

  .docs-placeholder {
    padding: 2rem;
    text-align: center;
    color: #666;
    background: #f5f5f5;
    border-radius: 4px;
  }
}
</style>