<template>
  <div class="document-sidebar">
    <div class="search-section">
      <input
        type="text"
        v-model="searchQuery"
        :placeholder="$t('search_placeholder')"
        class="search-input"
      />
    </div>
    
    <div class="filter-section">
      <select v-model="selectedType" class="filter-select">
        <option value="all">{{ $t('all_types') }}</option>
        <option value="module">{{ $t('modules') }}</option>
        <option value="class">{{ $t('classes') }}</option>
        <option value="trait">{{ $t('traits') }}</option>
      </select>
    </div>
    
    <div class="tree-section">
      <div v-for="item in filteredItems" :key="item.path" class="tree-item">
        <div 
          class="item-header"
          :class="{ 'active': item.path === currentPath }"
          @click="navigateToItem(item)"
        >
          <span class="item-type">{{ item.type }}</span>
          <span class="item-name">{{ item.name }}</span>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { useFluent } from 'fluent-vue'

const { $t } = useFluent()
const route = useRoute()
const router = useRouter()

const searchQuery = ref('')
const selectedType = ref('all')
const currentPath = computed(() => route.params.modulePath as string)

// 模拟数据，实际项目中应该从API获取
const items = ref([
  {
    type: 'module',
    name: 'collections',
    path: 'std/collections',
    description: 'Collection types provided by the standard library'
  },
  {
    type: 'class',
    name: 'HashMap',
    path: 'std/collections/hash_map',
    description: 'A hash map implementation'
  },
  {
    type: 'trait',
    name: 'Iterator',
    path: 'std/iter/trait.Iterator',
    description: 'An interface for dealing with iterators'
  }
])

const filteredItems = computed(() => {
  return items.value.filter(item => {
    const matchesSearch = item.name.toLowerCase().includes(searchQuery.value.toLowerCase()) ||
                         item.description.toLowerCase().includes(searchQuery.value.toLowerCase())
    const matchesType = selectedType.value === 'all' || item.type === selectedType.value
    return matchesSearch && matchesType
  })
})

const navigateToItem = (item: any) => {
  router.push({
    name: 'document',
    params: {
      ...route.params,
      modulePath: item.path
    }
  })
}
</script>

<style lang="scss" scoped>
.document-sidebar {
  width: 300px;
  height: 100%;
  border-right: 1px solid #eee;
  background: #fff;
  padding: 1rem;
  
  .search-section {
    margin-bottom: 1rem;
    
    .search-input {
      width: 100%;
      padding: 0.5rem;
      border: 1px solid #ddd;
      border-radius: 4px;
      font-size: 0.9rem;
      
      &:focus {
        outline: none;
        border-color: #007bff;
      }
    }
  }
  
  .filter-section {
    margin-bottom: 1rem;
    
    .filter-select {
      width: 100%;
      padding: 0.5rem;
      border: 1px solid #ddd;
      border-radius: 4px;
      font-size: 0.9rem;
      background: #fff;
      
      &:focus {
        outline: none;
        border-color: #007bff;
      }
    }
  }
  
  .tree-section {
    .tree-item {
      margin-bottom: 0.5rem;
      
      .item-header {
        display: flex;
        align-items: center;
        padding: 0.5rem;
        border-radius: 4px;
        cursor: pointer;
        
        &:hover {
          background: #f8f9fa;
        }
        
        &.active {
          background: #e9ecef;
        }
        
        .item-type {
          font-size: 0.8rem;
          padding: 0.2rem 0.4rem;
          background: #e9ecef;
          border-radius: 3px;
          margin-right: 0.5rem;
          color: #495057;
        }
        
        .item-name {
          font-size: 0.9rem;
          color: #333;
        }
      }
    }
  }
}
</style>