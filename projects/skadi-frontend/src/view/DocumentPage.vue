<template>
  <top-navigation/>
  <div class="document-page">
    <document-sidebar class="sidebar"/>
    <div class="document-container">
      <Suspense>
        <template #default>
          <div v-if="documentInfo">
            <module-view v-if="documentType === 'module'" :moduleName="moduleName" :version="version"/>
            <class-view v-else-if="documentType === 'class'" :className="className" :version="version"/>
            <trait-view v-else-if="documentType === 'trait'" :traitName="traitName" :version="version"/>
          </div>
          <div class="error" v-else-if="error">{{ error }}</div>
        </template>
        <template #fallback>
          <div class="loading">{{ $t('loading') }}</div>
        </template>
      </Suspense>
    </div>
  </div>
</template>

<script setup lang="ts">
import {onMounted, ref} from 'vue'
import {useRoute} from 'vue-router'
import {useFluent} from 'fluent-vue'
import TopNavigation from '@/components/TopNavigation.vue'
import {ClassView, ModuleView, TraitView} from '@/components/document'
import DocumentSidebar from '@/components/document/DocumentSidebar.vue'
import {documentQueryByPath} from "@/api/api-document.ts";

const {$t} = useFluent()
const route = useRoute()

const organization = ref(route.params.organization as string)
const library = ref(route.params.library as string)
const version = ref(route.params.version as string)
const modulePath = ref(route.params.modulePath as string)

const documentType = ref('module')
const documentInfo = ref({
  name: 'std::collections',
  version: '1.0.0',
  description: 'Collection types provided by the standard library.',
  items: [
    {
      type: 'Module',
      name: 'hash_map',
      description: 'A hash map implemented with linear probing and Robin Hood bucket stealing.'
    },
    {
      type: 'Module',
      name: 'vec_deque',
      description: 'A double-ended queue implemented with a growable ring buffer.'
    },
    {
      type: 'Class',
      name: 'BTreeMap',
      description: 'A map based on a B-Tree.'
    },
    {
      type: 'Trait',
      name: 'FromIterator',
      description: 'Conversion from an Iterator.'
    }
  ]
})
const error = ref('')

const moduleName = ref(modulePath.value)
const className = ref('')
const traitName = ref('')

const fetchDocumentInfo = async () => {
  try {
    error.value = ''
    // 在实际项目中，这里会调用 API 获取文档信息
    // 目前使用模拟数据
    documentInfo.value = documentInfo.value
  } catch (e) {
    error.value = 'Failed to load documentation'
  }
}

onMounted(() => {
  fetchDocumentInfo()
})
</script>

<style scoped lang="scss">
.document-page {
  display: flex;
  height: calc(100vh - 60px);

  .sidebar {
    flex-shrink: 0;
  }

  .document-container {
    flex: 1;
    padding: 2rem;
    background: #fff;
    border-radius: 8px;
    box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
    overflow-y: auto;
  }

  .loading {
    padding: 2rem;
    text-align: center;
    color: #666;
  }

  .error {
    padding: 2rem;
    text-align: center;
    color: #dc3545;
  }
}
</style>