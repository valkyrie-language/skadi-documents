<script setup lang="ts">
import {ref} from 'vue'
import {useFluent} from 'fluent-vue'
import {useRoute} from 'vue-router'
import PackageHeader from '../components/PackageHeader.vue'
import InstallCommand from '../components/InstallCommand.vue'
import TabNavigation from '../components/TabNavigation.vue'
import PackageContent from '../components/PackageContent.vue'

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
      <PackageHeader :package-info="packageInfo" />
      <InstallCommand :package-name="packageInfo.name" />
      <TabNavigation :tabs="tabs" v-model:active-tab="activeTab" />
      <PackageContent :active-tab="activeTab" :package-info="packageInfo" />
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
}
</style>