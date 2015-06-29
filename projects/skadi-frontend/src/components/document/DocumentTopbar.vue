<template>
  <nav class="top-navigation">
    <div class="nav-content">
      <div class="left-section">
        <div class="logo" @click="navigateHome">
          <img src="/logo.svg" alt="Logo" class="logo">
        </div>
        <document-search class="search-box"/>
      </div>
      <div class="right-section">
        <a href="#" class="nav-link">Browse packages</a>
        <a href="#" class="nav-link">Docs</a>
        <a href="#" class="nav-link">Sign in</a>
      </div>
    </div>
  </nav>
</template>

<script setup lang="ts">
import {useRouter} from 'vue-router'
import DocumentSearch from "./DocumentSearch.vue";

const router = useRouter()

const navigateHome = () => {
  router.push('/')
}
</script>


<style lang="scss" scoped>
.top-navigation {
  background-color: #ffffff;
  border-bottom: 1px solid #e5e7eb;
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  z-index: 100;
  height: 64px;
  display: flex;
  align-items: center;

  .nav-content {
    max-width: 1200px;
    margin: 0 auto;
    padding: 0 1rem;
    width: 100%;
    display: flex;
    justify-content: space-between;
    align-items: center;
  }

  .left-section {
    display: flex;
    align-items: center;
    gap: 1rem;

    .logo {
      cursor: pointer;
      height: 32px;
      width: auto;
    }

    .search-box {
      width: 300px;
    }
  }

  .right-section {
    display: flex;
    align-items: center;
    gap: 1.5rem;

    .nav-link {
      color: #4b5563;
      text-decoration: none;
      font-size: 0.875rem;
      font-weight: 500;

      &:hover {
        color: #1f2937;
      }
    }
  }
}
</style>