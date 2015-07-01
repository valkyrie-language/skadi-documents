<template>
  <document-topbar/>
  <div class="document-page">
    <document-sidebar class="sidebar"/>
    <div class="document-container">
      <Suspense>
        <template #default>
          <div v-if="documentInfo">
            <module-view v-if="documentType === 'module'" :module-info="documentInfo" :version="version"/>
            <class-view v-else-if="documentType === 'class'" :class-info="documentInfo" :version="version"/>
            <trait-view v-else-if="documentType === 'trait'" :trait-info="documentInfo" :version="version"/>
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
import {computed, onMounted, ref} from 'vue'
import {useRoute} from 'vue-router'
import {useFluent} from 'fluent-vue'
import {ClassView, DocumentSidebar, DocumentTopbar, ModuleView, TraitView} from '@/components/document'
import {documentQueryByPath} from "@/api/api-document.ts";
import {type DocumentInfo, ModuleType} from "@/api/models";

const {$t} = useFluent()
const route = useRoute()

const organization = ref(route.params.organization as string)
const library = ref(route.params.package as string)
const version = ref(route.params.version as string)
const modulePath = ref(route.params.module_path as string)

const documentInfo = ref<DocumentInfo | null>(null)
const documentType = computed(() => {
  return documentInfo.value.type as ModuleType
})
const error = ref('')

const fetchDocumentInfo = async () => {
  try {
    error.value = ''
    const response = await documentQueryByPath({
      organization: organization.value,
      library: library.value,
      version: version.value,
      module_path: modulePath.value
    })
    documentInfo.value = response
  } catch (e) {
    error.value = '加载文档失败'
    console.error('Failed to load documentation:', e)
  }
}

onMounted(() => {
  fetchDocumentInfo()
})
</script>

<style scoped lang="scss">
.document-page {
  display: flex;
  height: calc(100vh - 64px);
  margin-top: 64px;

  .sidebar {
    flex-shrink: 0;
    height: 100%;
    overflow-y: auto;
    border-right: 1px solid #e5e7eb;
  }

  .document-container {
    flex: 1;
    height: 100%;
    background: #fff;
    overflow-y: auto;

    &::-webkit-scrollbar {
      width: 8px;
    }

    &::-webkit-scrollbar-track {
      background: #f1f1f1;
    }

    &::-webkit-scrollbar-thumb {
      background: #888;
      border-radius: 4px;
    }
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