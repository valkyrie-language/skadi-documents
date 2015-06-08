<script setup lang="ts">
import { ref } from 'vue'
import { useRoute } from 'vue-router'

const route = useRoute()
const orgName = route.params.organization as string

// 模拟组织数据，实际项目中应从API获取
const orgInfo = {
  name: orgName,
  description: 'A collection of high-quality packages',
  packages: 15,
  members: 8,
  avatar: 'https://www.gravatar.com/avatar/00000000000000000000000000000000?d=mp&f=y'
}

// 模拟包列表数据
const packages = ref([
  {
    name: `@${orgName}/package-one`,
    version: '1.0.0',
    description: 'First package description',
    downloads: '2.3k',
    lastUpdate: '2024-01-25'
  },
  {
    name: `@${orgName}/package-two`,
    version: '0.5.0',
    description: 'Second package description',
    downloads: '1.1k',
    lastUpdate: '2024-01-20'
  }
])

// 分页相关
const currentPage = ref(1)
const totalPages = ref(5)

const changePage = (page: number) => {
  currentPage.value = page
  // 实际项目中这里需要调用API获取对应页的数据
}
</script>

<template>
  <div class="organization-page">
    <div class="container">
      <!-- 组织信息头部 -->
      <div class="org-header">
        <img :src="orgInfo.avatar" :alt="orgInfo.name" class="org-avatar">
        <div class="org-info">
          <h1>{{ orgInfo.name }}</h1>
          <p class="description">{{ orgInfo.description }}</p>
          <div class="stats">
            <span>{{ orgInfo.packages }} packages</span>
            <span>{{ orgInfo.members }} members</span>
          </div>
        </div>
      </div>

      <!-- 包列表 -->
      <div class="packages-list">
        <div v-for="pkg in packages" :key="pkg.name" class="package-item">
          <div class="package-header">
            <h3>
              <router-link :to="`/@${orgName}/${pkg.name.split('/')[1]}`">{{ pkg.name }}</router-link>
            </h3>
            <span class="version">v{{ pkg.version }}</span>
          </div>
          <p class="description">{{ pkg.description }}</p>
          <div class="package-meta">
            <span>{{ pkg.downloads }} weekly downloads</span>
            <span>Last updated {{ pkg.lastUpdate }}</span>
          </div>
        </div>
      </div>

      <!-- 分页 -->
      <div class="pagination">
        <button 
          :disabled="currentPage === 1"
          @click="changePage(currentPage - 1)"
        >
          Previous
        </button>
        <span>Page {{ currentPage }} of {{ totalPages }}</span>
        <button 
          :disabled="currentPage === totalPages"
          @click="changePage(currentPage + 1)"
        >
          Next
        </button>
      </div>
    </div>
  </div>
</template>

<style lang="scss" scoped>
.organization-page {
  min-height: 100vh;
  background-color: #f8f9fa;
  padding: 2rem 0;

  .container {
    max-width: 1200px;
    margin: 0 auto;
    padding: 0 1rem;
  }

  .org-header {
    display: flex;
    align-items: center;
    gap: 2rem;
    margin-bottom: 3rem;
    padding: 2rem;
    background-color: white;
    border-radius: 8px;
    box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);

    .org-avatar {
      width: 100px;
      height: 100px;
      border-radius: 50%;
    }

    .org-info {
      h1 {
        margin: 0 0 0.5rem;
        font-size: 2rem;
        color: #212529;
      }

      .description {
        margin: 0 0 1rem;
        color: #6c757d;
      }

      .stats {
        display: flex;
        gap: 1rem;
        color: #495057;

        span {
          display: inline-flex;
          align-items: center;
          gap: 0.5rem;
        }
      }
    }
  }

  .packages-list {
    display: grid;
    gap: 1rem;
    margin-bottom: 2rem;

    .package-item {
      padding: 1.5rem;
      background-color: white;
      border-radius: 8px;
      box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);

      .package-header {
        display: flex;
        align-items: center;
        justify-content: space-between;
        margin-bottom: 0.5rem;

        h3 {
          margin: 0;
          font-size: 1.25rem;

          a {
            color: #0d6efd;
            text-decoration: none;

            &:hover {
              text-decoration: underline;
            }
          }
        }

        .version {
          color: #6c757d;
          font-size: 0.875rem;
        }
      }

      .description {
        margin: 0 0 1rem;
        color: #495057;
      }

      .package-meta {
        display: flex;
        gap: 1rem;
        color: #6c757d;
        font-size: 0.875rem;
      }
    }
  }

  .pagination {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 1rem;

    button {
      padding: 0.5rem 1rem;
      border: 1px solid #dee2e6;
      border-radius: 4px;
      background-color: white;
      color: #212529;
      cursor: pointer;

      &:disabled {
        opacity: 0.5;
        cursor: not-allowed;
      }

      &:not(:disabled):hover {
        background-color: #e9ecef;
      }
    }

    span {
      color: #6c757d;
    }
  }
}
</style>