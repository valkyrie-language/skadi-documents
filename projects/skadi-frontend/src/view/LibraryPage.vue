<script setup lang="ts">
import {ref} from 'vue'
import {useFluent} from 'fluent-vue'
import {useRoute} from 'vue-router'

const {$t} = useFluent()
const route = useRoute()

// 从路由参数获取包信息
const orgName = route.params.organization as string
const libName = route.params.library as string

// 模拟包数据，实际项目中应从API获取
const packageInfo = {
  name: `@${orgName}/${libName}`,
  version: '0.1.2',
  description: 'A image encoder/decoder for the PNG format',
  author: 'BlackAsLight',
  license: 'MIT',
  repository: 'https://github.com/BlackAsLight/img',
  downloads: '1.2k',
  lastUpdate: '2024-01-20'
}

const tabs = [
  {
    id: 'overview',
    label: 'Overview'
  },
  {
    id: 'docs',
    label: 'Docs'
  },
  {
    id: 'files',
    label: 'Files'
  },
  {
    id: 'dependencies',
    label: 'Dependencies'
  },
  {
    id: 'dependents',
    label: 'Dependents'
  }
]

// 当前选中的标签页
const activeTab = ref('overview')
</script>

<template>
  <div class="library-page">
    <div class="container">
      <!-- 包信息区域 -->
      <div class="package-info">
        <h1>{{ packageInfo.name }}</h1>
        <p class="description">{{ packageInfo.description }}</p>

        <div class="meta-info">
          <div class="version">
            <span class="label">Version</span>
            <span class="value">{{ packageInfo.version }}</span>
          </div>
          <div class="license">
            <span class="label">License</span>
            <span class="value">{{ packageInfo.license }}</span>
          </div>
          <div class="downloads">
            <span class="label">Downloads</span>
            <span class="value">{{ packageInfo.downloads }}</span>
          </div>
          <div class="last-update">
            <span class="label">Last Update</span>
            <span class="value">{{ packageInfo.lastUpdate }}</span>
          </div>
        </div>

        <div class="installation">
          <div class="install-command">
            <code>deno add {{ packageInfo.name }}</code>
            <button class="copy-button">Copy</button>
          </div>
        </div>
      </div>

      <!-- 导航栏 -->
      <nav class="tabs">
        <button
          v-for="tab in tabs"
          :key="tab.id"
          :class="['tab-button', { active: activeTab === tab.id }]"
          @click="activeTab = tab.id"
        >
          {{ tab.label }}
        </button>
      </nav>

      <!-- 内容区域 -->
      <div class="content-area">
        <!-- 根据activeTab显示不同内容 -->
        <div v-if="activeTab === 'overview'" class="overview-content">
          <h2>Overview</h2>
          <!-- 实际内容根据API返回数据展示 -->
          <p>This is a TypeScript implementation of the PNG image format. The module offers encoding and
            decoding abilities. The raw pixel format/ the decoded format is a repeating sequence of [r, g,
            b, a] in a Uint8Array.</p>
        </div>
      </div>
    </div>
  </div>
</template>

<style lang="scss" scoped>
.library-page {
  min-height: 100vh;
  background-color: #f8f9fa;
  padding: 2rem 0;

  .container {
    max-width: 1200px;
    margin: 0 auto;
    padding: 0 1rem;
  }

  .package-info {
    background-color: white;
    border-radius: 8px;
    padding: 2rem;
    margin-bottom: 2rem;
    box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);

    h1 {
      font-size: 2rem;
      color: #212529;
      margin: 0 0 1rem;
    }

    .description {
      font-size: 1.1rem;
      color: #495057;
      margin-bottom: 1.5rem;
    }

    .meta-info {
      display: grid;
      grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
      gap: 1rem;
      margin-bottom: 1.5rem;

      .label {
        display: block;
        font-size: 0.875rem;
        color: #868e96;
        margin-bottom: 0.25rem;
      }

      .value {
        font-size: 1rem;
        color: #212529;
        font-weight: 500;
      }
    }

    .installation {
      .install-command {
        display: flex;
        align-items: center;
        background-color: #f8f9fa;
        padding: 1rem;
        border-radius: 4px;

        code {
          flex: 1;
          font-family: monospace;
          color: #212529;
        }

        .copy-button {
          padding: 0.5rem 1rem;
          background-color: #4dabf7;
          color: white;
          border: none;
          border-radius: 4px;
          cursor: pointer;
          transition: background-color 0.2s;

          &:hover {
            background-color: #339af0;
          }
        }
      }
    }
  }

  .tabs {
    display: flex;
    gap: 1rem;
    margin-bottom: 2rem;
    border-bottom: 2px solid #e9ecef;
    padding-bottom: 1rem;

    .tab-button {
      padding: 0.5rem 1rem;
      background: none;
      border: none;
      color: #495057;
      font-size: 1rem;
      cursor: pointer;
      transition: all 0.2s;

      &:hover {
        color: #212529;
      }

      &.active {
        color: #4dabf7;
        font-weight: 500;
        position: relative;

        &::after {
          content: '';
          position: absolute;
          bottom: -1rem;
          left: 0;
          width: 100%;
          height: 2px;
          background-color: #4dabf7;
        }
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
  }
}
</style>