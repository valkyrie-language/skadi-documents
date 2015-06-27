<template>
  <top-navigation/>
  <div class="package-page">
    <div class="package-container">
      <Suspense>
        <template #default>
          <div v-if="packageInfo">
            <package-header
              :packageInfo="packageInfo"
            />
            <package-content
              v-model:activeTab="activeTab"
              :packageInfo="packageInfo"
            />
          </div>
          <div class="error" v-else-if="error">{{ error }}</div>
        </template>
        <template #fallback>
          <div class="loading">{{ $t('loading') }}</div>
        </template>
      </Suspense>
    </div>
  </div>
</template>

<script setup lang="ts">
import {onMounted, ref} from 'vue'
import {useRoute} from 'vue-router'
import {useFluent} from 'fluent-vue'
import {packageQueryByName} from '../api/api-package'
import type {PackageDetail} from '../api/models'
import TopNavigation from "@/components/TopNavigation.vue";
import {PackageHeader, PackageContent} from "@/components/package"
const {$t} = useFluent()

const activeTab = ref('overview')

const route = useRoute()
const organization = ref(route.params.organization as string)
const name = ref(route.params.package as string)
const packageInfo = ref<PackageDetail | null>(null)
const loading = ref(true)
const error = ref('')

const fetchPackageInfo = async () => {
  try {
    loading.value = true
    error.value = ''
    const response = await packageQueryByName({
      organization: organization.value,
      name: name.value
    })
    packageInfo.value = response
  } catch (e) {
    error.value = 'Failed to load package information'
  } finally {
    loading.value = false
  }
}

const formatDate = (date: string) => {
  return new Date(date).toLocaleDateString()
}

onMounted(() => {
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