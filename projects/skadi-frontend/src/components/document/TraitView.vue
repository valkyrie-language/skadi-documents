<template>
  <div class="trait-view">
    <div class="trait-header">
      <h1>{{ traitName }}</h1>
      <div class="trait-meta">
        <span class="version">{{ version }}</span>
      </div>
    </div>
    <div class="trait-content">
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
        <h2>Required Methods</h2>
        <div class="methods-list">
          <div v-for="method in requiredMethods" :key="method.name" class="method">
            <div class="method-header">
              <span class="method-name">{{ method.name }}</span>
              <span class="method-params">{{ method.params }}</span>
              <span class="method-return">{{ method.returnType }}</span>
            </div>
            <div class="method-description" v-if="method.description">
              {{ method.description }}
            </div>
          </div>
        </div>
      </div>
      
      <div class="doc-section">
        <h2>Provided Methods</h2>
        <div class="methods-list">
          <div v-for="method in providedMethods" :key="method.name" class="method">
            <div class="method-header">
              <span class="method-name">{{ method.name }}</span>
              <span class="method-params">{{ method.params }}</span>
              <span class="method-return">{{ method.returnType }}</span>
            </div>
            <div class="method-description" v-if="method.description">
              {{ method.description }}
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
const traitName = ref(route.params.trait as string)
const version = ref(route.params.version as string)
const description = ref('')
const requiredMethods = ref([])
const providedMethods = ref([])

// TODO: Fetch trait documentation data
</script>

<style lang="scss" scoped>
.trait-view {
  padding: 2rem;
  
  .trait-header {
    margin-bottom: 2rem;
    
    h1 {
      font-size: 2rem;
      margin-bottom: 0.5rem;
    }
    
    .trait-meta {
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
  
  .methods-list {
    .method {
      padding: 1rem;
      border: 1px solid #eee;
      border-radius: 4px;
      margin-bottom: 1rem;
      
      &:hover {
        background: #f8f9fa;
      }
      
      .method-header {
        display: flex;
        align-items: center;
        gap: 1rem;
        margin-bottom: 0.5rem;
        
        .method-name {
          font-weight: 500;
          color: #333;
        }
        
        .method-params,
        .method-return {
          color: #666;
          font-size: 0.9rem;
        }
      }
      
      .method-description {
        color: #666;
        font-size: 0.9rem;
      }
    }
  }
}
</style>