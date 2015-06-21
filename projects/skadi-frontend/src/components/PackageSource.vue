<template>
  <div class="files-content">
    <h2>{{ $t('package-files') }}</h2>
    <!-- 文件列表将在此处展示 -->
    <div class="files-placeholder">
      <p>{{ $t('files-coming-soon') }}</p>
    </div>
  </div>
</template>

<script setup lang="ts">
import { useFluent } from 'fluent-vue'
import type { PackageInfo } from '@/api/models'

defineProps<{
  packageInfo: PackageInfo
}>()

const { $t } = useFluent()
</script>

<style lang="scss" scoped>
.files-content {
  padding: 1rem;
  
  h2 {
    margin-bottom: 1rem;
    font-size: 1.5rem;
    color: #333;
  }

  .files-placeholder {
    padding: 2rem;
    text-align: center;
    color: #666;
    background: #f5f5f5;
    border-radius: 4px;
  }
}
</style>