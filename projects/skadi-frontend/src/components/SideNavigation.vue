<script setup lang="ts">
import { ref } from 'vue'

const searchSymbol = ref('')

const sections = [
  {
    title: 'Add Package',
    items: [
      { label: 'deno add @a/b', copyText: 'deno add @a/b' },
      { label: 'Import symbol', copyText: 'import * as pkg from "@a/b";' }
    ]
  },
  {
    title: 'Document Navigation',
    items: [
      { label: 'Examples', href: '#examples' },
      { label: 'API Reference', href: '#api-reference' },
      { label: 'Installation', href: '#installation' }
    ]
  }
]

const copyToClipboard = (text: string) => {
  navigator.clipboard.writeText(text)
}
</script>

<template>
  <aside class="side-navigation">
    <div class="search-section">
      <input
        type="text"
        v-model="searchSymbol"
        placeholder="Search for symbols (Ctrl+/)"
        class="symbol-search"
      >
    </div>

    <div class="nav-sections">
      <div v-for="section in sections" :key="section.title" class="section">
        <h3 class="section-title">{{ section.title }}</h3>
        <ul class="section-items">
          <li v-for="item in section.items" :key="item.label" class="section-item">
            <template v-if="item.copyText">
              <button 
                class="copy-button"
                @click="copyToClipboard(item.copyText)"
              >
                {{ item.label }}
              </button>
            </template>
            <template v-else>
              <a :href="item.href" class="nav-link">{{ item.label }}</a>
            </template>
          </li>
        </ul>
      </div>
    </div>
  </aside>
</template>

<style lang="scss" scoped>
.side-navigation {
  width: 280px;
  background-color: #ffffff;
  border-left: 1px solid #e5e7eb;
  height: 100vh;
  position: fixed;
  top: 64px;
  right: 0;
  padding: 1.5rem;
  overflow-y: auto;

  .search-section {
    margin-bottom: 1.5rem;

    .symbol-search {
      width: 100%;
      padding: 0.5rem;
      border: 1px solid #e5e7eb;
      border-radius: 0.375rem;
      font-size: 0.875rem;
      outline: none;

      &:focus {
        border-color: #3b82f6;
        box-shadow: 0 0 0 1px #3b82f6;
      }
    }
  }

  .nav-sections {
    .section {
      margin-bottom: 2rem;

      &:last-child {
        margin-bottom: 0;
      }

      .section-title {
        font-size: 0.875rem;
        font-weight: 600;
        color: #4b5563;
        margin-bottom: 0.75rem;
      }

      .section-items {
        list-style: none;
        padding: 0;
        margin: 0;

        .section-item {
          margin-bottom: 0.5rem;

          &:last-child {
            margin-bottom: 0;
          }

          .copy-button {
            width: 100%;
            text-align: left;
            padding: 0.5rem;
            background-color: #f3f4f6;
            border: none;
            border-radius: 0.375rem;
            font-size: 0.875rem;
            color: #374151;
            cursor: pointer;
            transition: background-color 0.2s;

            &:hover {
              background-color: #e5e7eb;
            }
          }

          .nav-link {
            display: block;
            padding: 0.5rem;
            color: #4b5563;
            text-decoration: none;
            font-size: 0.875rem;
            border-radius: 0.375rem;

            &:hover {
              background-color: #f3f4f6;
            }
          }
        }
      }
    }
  }
}
</style>