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
import {ref} from 'vue'
import {useRoute} from 'vue-router'
import type {ClassInfo} from "@/api/models";

const route = useRoute()
const className = ref(route.params.class as string)
const version = ref(route.params.version as string)
const description = ref('')
const methods = ref([])
const properties = ref([])

const props = defineProps<{
  classInfo: ClassInfo
}>()
</script>

<style lang="scss" scoped>
.class-view {
  max-width: 960px;
  margin: 0 auto;
  padding: 1rem;

  .class-header {
    margin-bottom: 2rem;
    border-bottom: 1px solid #e1e4e8;
    padding-bottom: 1rem;

    h1 {
      font-size: 2rem;
      font-weight: 600;
      color: #000;
      margin-bottom: 0.5rem;
      font-family: "Source Serif Pro", serif;
    }

    .class-meta {
      display: flex;
      align-items: center;
      gap: 0.5rem;

      .version {
        color: #6e6e6e;
        font-size: 0.875rem;
        font-family: "Source Code Pro", monospace;
      }
    }
  }

  .doc-section {
    margin-bottom: 2rem;

    h2 {
      font-size: 1.5rem;
      font-weight: 600;
      color: #000;
      margin-bottom: 1rem;
      padding-bottom: 0.5rem;
      border-bottom: 1px solid #eaecef;
    }
  }

  .methods-list,
  .properties-list {
    .method,
    .property {
      padding: 1rem;
      background: #fafafa;
      border-radius: 3px;
      margin-bottom: 1rem;

      &:hover {
        background: #f5f5f5;
      }

      .method-header,
      .property-header {
        display: flex;
        align-items: center;
        flex-wrap: wrap;
        gap: 0.5rem;
        margin-bottom: 0.75rem;
        font-family: "Source Code Pro", monospace;

        .method-visibility,
        .property-visibility {
          color: #2b7489;
          font-size: 0.875rem;
        }

        .method-name,
        .property-name {
          font-weight: 600;
          color: #000;
        }

        .method-params,
        .method-return,
        .property-type {
          color: #6e6e6e;
          font-size: 0.875rem;
        }
      }

      .method-description,
      .property-description {
        color: #24292e;
        font-size: 0.875rem;
        line-height: 1.5;
      }
    }
  }

  .no-description {
    color: #6a737d;
    font-style: italic;
  }
}
</style>