<template>
  <aside class="sidebar">
    <div class="sidebar-section">
      <h3>{{ $t('usage') }}</h3>
      <div class="usage-example">
        <code>legion add @{{ packageInfo.organization }}/{{ packageInfo.name }}</code>
      </div>
    </div>
    <div class="sidebar-section">
      <h3>{{ $t('symbols') }}</h3>
      <ul class="symbol-list">
        <li>decodePNG</li>
        <li>encodePNG</li>
        <li>PNGOptions</li>
      </ul>
    </div>
  </aside>
</template>
<script setup lang="ts">

import type {PackageDetail} from "@/api/models";

const props = defineProps<{
  installer: string
  packageInfo: PackageDetail
}>()



</script>