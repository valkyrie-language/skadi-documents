<template>
  <div class="overview-content">
    <h2>{{ $t('package-overview') }}</h2>
    <p>{{ packageInfo.description }}</p>
  </div>
</template>

<script setup lang="ts">
import { useFluent } from 'fluent-vue'
import type {PackageDetail} from '@/api/models'

defineProps<{
  packageInfo: PackageDetail
}>()

const { $t } = useFluent()
</script>

<style lang="scss" scoped>
.overview-content {
  padding: 1rem;
  
  h2 {
    margin-bottom: 1rem;
    font-size: 1.5rem;
    color: #333;
  }

  p {
    color: #666;
    line-height: 1.6;
  }
}
</style>