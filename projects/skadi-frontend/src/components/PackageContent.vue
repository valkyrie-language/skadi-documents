<template>
  <div class="package-content">
    <div class="main-content">
      <div class="content-wrapper">
        <div class="tabs">
          <button
            v-for="tab in tabs"
            :key="tab.id"
            :class="['tab-button', { active: activeTab === tab.id }]"
            @click="$emit('update:activeTab', tab.id)"
          >
            {{ $t(tab.label) }}
          </button>
        </div>

        <div class="content-area">
          <package-overview v-if="activeTab === 'overview'" :packageInfo="packageInfo"/>
          <package-versions v-else-if="activeTab === 'docs'" :packageInfo="packageInfo"/>
          <package-source v-else-if="activeTab === 'files'" :packageInfo="packageInfo"/>
          <package-dependencies v-else-if="activeTab === 'dependencies'" :packageInfo="packageInfo"/>
          <package-dependents v-else-if="activeTab === 'dependents'" :packageInfo="packageInfo"/>
        </div>
      </div>

      <package-sidebar :package-info="packageInfo"/>
    </div>
  </div>
</template>

<script setup lang="ts">
import {useFluent} from 'fluent-vue'
import {ref} from 'vue'
import type {PackageDetail,} from "@/api/models"
import PackageOverview from './PackageOverview.vue'
import PackageVersions from './PackageVersions.vue'
import PackageSource from './PackageSource.vue'
import PackageDependencies from './PackageDependencies.vue'
import PackageDependents from './PackageDependents.vue'
import PackageSidebar from "@/components/PackageSidebar.vue";

defineProps<{
  activeTab: string
  packageInfo: PackageDetail
}>()

const emit = defineEmits<{
  'update:activeTab': [value: string]
}>()

const {$t} = useFluent()
const tabs = ref([
  {
    id: 'overview',
    label: 'package-overview'
  },
  {
    id: 'docs',
    label: 'package-docs'
  },
  {
    id: 'files',
    label: 'package-files'
  },
  {
    id: 'dependencies',
    label: 'package-dependencies'
  },
  {
    id: 'dependents',
    label: 'package-dependents'
  }
])
</script>


<style lang="scss" scoped>
.package-content {
  max-width: 1200px;
  margin: 0 auto;
  padding: 2rem;

  .package-header {
    margin-bottom: 2rem;

    .header-top {
      display: flex;
      justify-content: space-between;
      align-items: center;
      margin-bottom: 0.5rem;

      h1 {
        font-size: 2rem;
        color: #212529;
        margin: 0;
      }

      .search-box {
        width: 300px;
      }
    }

    .description {
      color: #666;
      font-size: 1.1rem;
    }
  }

  .main-content {
    display: grid;
    grid-template-columns: 1fr 300px;
    gap: 2rem;
  }

  .content-wrapper {
    .tabs {
      display: flex;
      gap: 1rem;
      margin-bottom: 1rem;
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
      background-color: white;
      border-radius: 8px;
      padding: 2rem;
      box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);

      h2 {
        font-size: 1.5rem;
        color: #212529;
        margin: 0 0 1rem;
      }

      p {
        color: #666;
        line-height: 1.6;
      }
    }
  }

}
</style>