<template>
  <div class="package-page">
    <div class="package-container">
      <package-header
        v-if="packageInfo"
        :packageInfo="packageInfo"
      />
      <package-content
        v-if="packageInfo"
        activeTab="overview"
        :packageInfo="packageInfo"
      />
      <div class="loading" v-else-if="loading">{{ $t('loading') }}</div>
      <div class="error" v-else-if="error">{{ error }}</div>
    </div>
  </div>
</template>

<script setup lang="ts">
import {onMounted, ref} from 'vue'
import {useRoute} from 'vue-router'
import {useFluent} from 'fluent-vue'
import {packageQueryByName} from '../api/api-package'
import type {PackageInfo} from '../api/models'
import PackageHeader from '../components/PackageHeader.vue'
import PackageContent from '../components/PackageContent.vue'

const {$t} = useFluent()

const route = useRoute()
const organization = ref(route.params.organization as string)
const name = ref(route.params.package as string)
const packageInfo = ref<PackageInfo | null>(null)
const loading = ref(true)
const error = ref('')

const fetchPackageInfo = async () => {
  console.log('Starting fetchPackageInfo with params:', {
    organization: organization.value,
    name: name.value
  })
  try {
    loading.value = true
    error.value = ''
    const response = await packageQueryByName({
      organization: organization.value,
      name: name.value
    })
    console.log('fetchPackageInfo response:', response)
    packageInfo.value = response
  } catch (e) {
    console.error('fetchPackageInfo error:', e)
    error.value = 'Failed to load package information'
  } finally {
    loading.value = false
    console.log('fetchPackageInfo completed')
  }
}

const formatDate = (date: string) => {
  return new Date(date).toLocaleDateString()
}

onMounted(() => {
  console.log('LibraryPage component mounted')
  fetchPackageInfo()
})
</script>

<style scoped lang="scss">
.package-page {
  padding: 2rem;

  .package-container {
    max-width: 1200px;
    margin: 0 auto;
    background: #fff;
    border-radius: 8px;
    box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
  }

  .library-header {
    padding: 2rem;
    border-bottom: 1px solid #eee;

    h1 {
      margin: 0;
      font-size: 2rem;
      color: #333;
    }
  }

  .library-content {
    padding: 1.5rem;

    .info-section {
      display: flex;
      flex-direction: column;
      gap: 1rem;
      max-width: 800px;
      margin: 0 auto;

      .info-item {
        display: flex;
        align-items: baseline;
        padding: 0.75rem 1rem;
        background: #f8f9fa;
        border-radius: 4px;
        transition: background-color 0.2s;

        &:hover {
          background: #f1f3f5;
        }

        strong {
          color: #555;
          min-width: 100px;
          margin-right: 1rem;
        }

        a {
          color: #007bff;
          text-decoration: none;

          &:hover {
            text-decoration: underline;
          }
        }
      }
    }
  }

  .loading {
    padding: 2rem;
    text-align: center;
    color: #666;
  }

  .error {
    padding: 2rem;
    text-align: center;
    color: #dc3545;
  }
}
</style>