<template>
  <div class="class-view">
    <div class="class-header">
      <h1>{{ className }}</h1>
      <div class="class-meta">
        <span class="version">{{ version }}</span>
      </div>
    </div>
    <div class="class-content">
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
        <h2>Methods</h2>
        <div class="methods-list">
          <div v-for="method in methods" :key="method.name" class="method">
            <div class="method-header">
              <span class="method-visibility">{{ method.visibility }}</span>
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
        <h2>Properties</h2>
        <div class="properties-list">
          <div v-for="prop in properties" :key="prop.name" class="property">
            <div class="property-header">
              <span class="property-visibility">{{ prop.visibility }}</span>
              <span class="property-name">{{ prop.name }}</span>
              <span class="property-type">{{ prop.type }}</span>
            </div>
            <div class="property-description" v-if="prop.description">
              {{ prop.description }}
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
const className = ref(route.params.class as string)
const version = ref(route.params.version as string)
const description = ref('')
const methods = ref([])
const properties = ref([])

// TODO: Fetch class documentation data
</script>

<style lang="scss" scoped>
.class-view {
  padding: 2rem;
  
  .class-header {
    margin-bottom: 2rem;
    
    h1 {
      font-size: 2rem;
      margin-bottom: 0.5rem;
    }
    
    .class-meta {
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
  
  .methods-list,
  .properties-list {
    .method,
    .property {
      padding: 1rem;
      border: 1px solid #eee;
      border-radius: 4px;
      margin-bottom: 1rem;
      
      &:hover {
        background: #f8f9fa;
      }
      
      .method-header,
      .property-header {
        display: flex;
        align-items: center;
        gap: 1rem;
        margin-bottom: 0.5rem;
        
        .method-visibility,
        .property-visibility {
          background: #e9ecef;
          padding: 0.2rem 0.5rem;
          border-radius: 4px;
          font-size: 0.9rem;
          color: #495057;
        }
        
        .method-name,
        .property-name {
          font-weight: 500;
          color: #333;
        }
        
        .method-params,
        .method-return,
        .property-type {
          color: #666;
          font-size: 0.9rem;
        }
      }
      
      .method-description,
      .property-description {
        color: #666;
        font-size: 0.9rem;
      }
    }
  }
}
</style>