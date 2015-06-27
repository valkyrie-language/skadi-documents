<template>
  <home-navigation/>
  <div class="package-manager">
    <div class="container">
      <!-- Logo和标题区域 -->
      <div class="logo-section">
        <img src="/logo.svg" alt="JSR Logo" class="logo"/>
        <div class="title-section">
          <h1>The open-source package registry</h1>
          <p>for modern JavaScript and TypeScript</p>
        </div>
      </div>

      <!-- 搜索和发布区域 -->
      <div class="search-section">
        <SearchBox/>
        <button class="publish-button">+ Publish a package</button>
      </div>

      <!-- 包列表区域 -->
      <div class="package-lists">
        <PackageRecommend title="featured" :packages="featuredPackages"/>
        <PackageRecommend title="recently-updated" :packages="recentlyUpdatedPackages"/>
        <PackageRecommend title="new-to-jsr" :packages="newPackages"/>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import {useFluent} from 'fluent-vue'
import SearchBox from '../components/SearchBox.vue'
import PackageRecommend from '../components/package/PackageRecommend.vue'
import HomeNavigation from "@/components/HomeNavigation.vue";

const {$t} = useFluent()

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

<style lang="scss" scoped>
.package-manager {
  min-height: 100vh;
  background-color: #f8f9fa;
  padding: 2rem 0;

  .container {
    max-width: 1200px;
    margin: 0 auto;
    padding: 0 1rem;
  }

  .logo-section {
    text-align: center;
    margin-bottom: 2rem;

    .logo {
      width: 120px;
      height: auto;
      margin-bottom: 1rem;
    }

    .title-section {
      h1 {
        font-size: 2.5rem;
        color: #212529;
        margin: 0 0 0.5rem;
      }

      p {
        font-size: 1.25rem;
        color: #495057;
        margin: 0;
      }
    }
  }

  .search-section {
    display: flex;
    align-items: center;
    gap: 1rem;
    margin-bottom: 3rem;

    .publish-button {
      padding: 0.75rem 1.5rem;
      background-color: #ffd43b;
      color: #212529;
      border: none;
      border-radius: 4px;
      font-weight: 500;
      cursor: pointer;
      transition: background-color 0.2s;

      &:hover {
        background-color: #fcc419;
      }
    }
  }

  .package-lists {
    display: grid;
    grid-template-columns: repeat(3, 1fr);
    gap: 2rem;
  }
}
</style>

