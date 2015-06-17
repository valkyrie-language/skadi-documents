<template>
  <div class="package-content">
    <header class="package-header">
      <div class="header-top">
        <h1>{{ packageInfo.name }}</h1>
        <SearchBox class="search-box"/>
      </div>
      <p class="description">{{ packageInfo.description }}</p>
    </header>

    <div class="main-content">
      <div class="content-wrapper">
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

      <aside class="sidebar">
        <div class="sidebar-section">
          <h3>{{ $t('usage') }}</h3>
          <div class="usage-example">
            <code>deno add jsr:@img/png</code>
          </div>
        </div>
        <div class="sidebar-section">
          <h3>{{ $t('symbols') }}</h3>
          <ul class="symbol-list">
            <li>decodePNG</li>
            <li>encodePNG</li>
            <li>PNGOptions</li>
          </ul>
        </div>
      </aside>
    </div>
  </div>
</template>

<script setup lang="ts">
import {useFluent} from 'fluent-vue'
import {ref} from 'vue'
import type {PackageInfo} from "@/api/models"
import SearchBox from './SearchBox.vue';

defineProps<{
  activeTab: string
  packageInfo: PackageInfo
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

  .sidebar {
    .sidebar-section {
      background-color: white;
      border-radius: 8px;
      padding: 1.5rem;
      box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
      margin-bottom: 1rem;

      h3 {
        font-size: 1.2rem;
        color: #212529;
        margin: 0 0 1rem;
      }

      .usage-example {
        background-color: #f8f9fa;
        border-radius: 4px;
        padding: 1rem;
        margin-bottom: 1rem;

        code {
          font-family: 'SFMono-Regular', Consolas, 'Liberation Mono', Menlo, monospace;
          color: #0969da;
        }
      }

      .symbol-list {
        list-style: none;
        padding: 0;
        margin: 0;

        li {
          color: #0969da;
          padding: 0.5rem 0;
          border-bottom: 1px solid #eee;
          cursor: pointer;

          &:last-child {
            border-bottom: none;
          }

          &:hover {
            color: #0366d6;
          }
        }
      }
    }
  }
}
</style>