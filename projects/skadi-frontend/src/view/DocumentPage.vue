<template>
  <top-navigation/>
  <div class="document-page">
    <document-sidebar class="sidebar"/>
    <div class="document-container">
      <Suspense>
        <template #default>
          <div v-if="documentInfo">
            <module-view v-if="documentType === 'module'" :moduleInfo="documentInfo" :version="version"/>
            <class-view v-else-if="documentType === 'class'" :className="itemName" :version="version"/>
            <trait-view v-else-if="documentType === 'trait'" :traitName="itemName" :version="version"/>
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
import TopNavigation from '@/components/TopNavigation.vue'
import {ClassView, ModuleView, TraitView} from '@/components/document'
import DocumentSidebar from '@/components/document/DocumentSidebar.vue'
import {documentQueryByPath} from "@/api/api-document.ts";
import type {DocumentInfo} from "@/api/models";

const {$t} = useFluent()
const route = useRoute()

const organization = ref(route.params.organization as string)
const library = ref(route.params.package as string)
const version = ref(route.params.version as string)
const modulePath = ref(route.params.module_path as string)

const documentInfo = ref<DocumentInfo | null>(null)
const documentType = computed(() => {
  return documentInfo.value.type
})
const error = ref('')

const itemName = ref('')

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

    // 根据返回的文档类型设置相应的变量
    if (response) {
      documentType.value = response.type.toLowerCase()
      if (documentType.value === 'module') {
        itemName.value = modulePath.value
      } else if (documentType.value === 'class') {
        itemName.value = response.name
      } else if (documentType.value === 'trait') {
        itemName.value = response.name
      }
    }
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