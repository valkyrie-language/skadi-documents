<template>
  <div class="dependents-content">
    <h2>{{ $t('package-dependents') }}</h2>
    <!-- 被依赖列表将在此处展示 -->
    <div class="dependents-placeholder">
      <p>{{ $t('dependents-coming-soon') }}</p>
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
.dependents-content {
  padding: 1rem;
  
  h2 {
    margin-bottom: 1rem;
    font-size: 1.5rem;
    color: #333;
  }

  .dependents-placeholder {
    padding: 2rem;
    text-align: center;
    color: #666;
    background: #f5f5f5;
    border-radius: 4px;
  }
}
</style>