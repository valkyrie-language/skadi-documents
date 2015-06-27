<template>
  <div class="module-view">
    <div class="module-header">
      <h1>{{ moduleName }}</h1>
      <div class="module-meta">
        <span class="version">{{ version }}</span>
      </div>
    </div>
    <div class="module-content">
      <div class="doc-section">
        <h2>Description</h2>
        <div class="description" v-if="description">
          {{ description }}
        </div>
        <div class="no-description" v-else>
          No description available.
        </div>
      </div>
      
      <div class="doc-section">
        <h2>Items</h2>
        <div class="items-list">
          <div v-for="item in items" :key="item.name" class="item">
            <div class="item-header">
              <span class="item-type">{{ item.type }}</span>
              <span class="item-name">{{ item.name }}</span>
            </div>
            <div class="item-description" v-if="item.description">
              {{ item.description }}
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { useRoute } from 'vue-router'

const route = useRoute()
const moduleName = ref(route.params.module as string)
const version = ref(route.params.version as string)
const description = ref('')
const items = ref([])

// TODO: Fetch module documentation data
</script>

<style lang="scss" scoped>
.module-view {
  padding: 2rem;
  
  .module-header {
    margin-bottom: 2rem;
    
    h1 {
      font-size: 2rem;
      margin-bottom: 0.5rem;
    }
    
    .module-meta {
      color: #666;
      
      .version {
        background: #f0f0f0;
        padding: 0.2rem 0.5rem;
        border-radius: 4px;
        font-size: 0.9rem;
      }
    }
  }
  
  .doc-section {
    margin-bottom: 2rem;
    
    h2 {
      font-size: 1.5rem;
      margin-bottom: 1rem;
      color: #333;
    }
  }
  
  .items-list {
    .item {
      padding: 1rem;
      border: 1px solid #eee;
      border-radius: 4px;
      margin-bottom: 1rem;
      
      &:hover {
        background: #f8f9fa;
      }
      
      .item-header {
        display: flex;
        align-items: center;
        gap: 1rem;
        margin-bottom: 0.5rem;
        
        .item-type {
          background: #e9ecef;
          padding: 0.2rem 0.5rem;
          border-radius: 4px;
          font-size: 0.9rem;
          color: #495057;
        }
        
        .item-name {
          font-weight: 500;
          color: #333;
        }
      }
      
      .item-description {
        color: #666;
        font-size: 0.9rem;
      }
    }
  }
}
</style>