<template>
  <div class="dependencies-content">
    <h2>{{ $t('package-dependencies') }}</h2>
    <!-- 依赖列表将在此处展示 -->
    <div class="dependencies-placeholder">
      <p>{{ $t('dependencies-coming-soon') }}</p>
    </div>
  </div>
</template>

<script setup lang="ts">
import { useFluent } from 'fluent-vue'
import type {PackageDetail,} from '@/api/models'

defineProps<{
  packageInfo: PackageDetail
}>()

const { $t } = useFluent()
</script>

<style lang="scss" scoped>
.dependencies-content {
  padding: 1rem;
  
  h2 {
    margin-bottom: 1rem;
    font-size: 1.5rem;
    color: #333;
  }

  .dependencies-placeholder {
    padding: 2rem;
    text-align: center;
    color: #666;
    background: #f5f5f5;
    border-radius: 4px;
  }
}
</style>