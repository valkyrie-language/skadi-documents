<template>
  <div class="package-manager">
    <!-- 顶部导航栏 -->
    <header class="header">
      <div class="container">
        <nav class="home-navigation">
        </nav>
      </div>
    </header>
  </div>
</template>

<script setup lang="ts">
import { useFluent } from 'fluent-vue'
import SearchBox from '../components/SearchBox.vue'
import PackageList from '../components/PackageList.vue'

const { $t } = useFluent()

const featuredPackages = [
  {
    name: '@img/png',
    version: '1.0.0',
    description: '图像处理工具包'
  },
  {
    name: '@paoramen/cheer-reader',
    version: '2.1.0',
    description: '高性能文本解析器'
  },
  {
    name: '@valibot/valibot',
    version: '0.1.0',
    description: '轻量级验证库'
  }
]

const recentlyUpdatedPackages = [
  {
    name: '@goatdb/goatdb',
    version: '0.2.0',
    description: '现代化数据库解决方案'
  },
  {
    name: '@planigale/sse',
    version: '0.2.6',
    description: 'Server-Sent Events 客户端'
  },
  {
    name: '@omedia/mcp-server',
    version: '1.0.0-alpha',
    description: '媒体内容处理服务器'
  }
]

const newPackages = [
  {
    name: '@omedia/mcp-server-drupal',
    version: '0.1.0',
    description: 'Drupal媒体处理插件'
  },
  {
    name: '@reliverse/relidler-sdk',
    version: '0.1.0',
    description: '空闲时间任务调度SDK'
  },
  {
    name: '@jipaix/test',
    version: '0.1.0',
    description: '测试工具集'
  }
]
</script>

