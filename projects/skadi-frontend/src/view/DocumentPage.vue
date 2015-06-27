<template>
  <top-navigation/>
  <div class="document-page">
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

const {$t} = useFluent()
const route = useRoute()

const organization = ref(route.params.organization as string)
const library = ref(route.params.library as string)
const version = ref(route.params.version as string)
const modulePath = ref(route.params.modulePath as string)

const documentType = ref('module')
const documentInfo = ref(null)
const error = ref('')

const moduleName = ref('')
const className = ref('')
const traitName = ref('')

const fetchDocumentInfo = async () => {
  try {
    error.value = ''
    // TODO: Implement API call to fetch document info
    // const response = await fetchDocInfo({
    //   organization: organization.value,
    //   library: library.value,
    //   version: version.value,
    //   modulePath: modulePath.value
    // })
    // documentInfo.value = response
    // documentType.value = response.type
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
  padding: 2rem;

  .document-container {
    max-width: 1200px;
    margin: 0 auto;
    background: #fff;
    border-radius: 8px;
    box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
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