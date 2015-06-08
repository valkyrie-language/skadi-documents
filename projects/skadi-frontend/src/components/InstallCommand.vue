<script setup lang="ts">
import { ref } from 'vue'

defineProps<{
  packageName: string
}>()

const copied = ref(false)

const copyCommand = () => {
  const command = `deno add ${packageName}`
  navigator.clipboard.writeText(command)
  copied.value = true
  setTimeout(() => {
    copied.value = false
  }, 2000)
}
</script>

<template>
  <div class="installation">
    <div class="install-command">
      <code>deno add {{ packageName }}</code>
      <button class="copy-button" @click="copyCommand">
        {{ copied ? 'Copied!' : 'Copy' }}
      </button>
    </div>
  </div>
</template>

<style lang="scss" scoped>
.installation {
  .install-command {
    display: flex;
    align-items: center;
    background-color: #f8f9fa;
    padding: 1rem;
    border-radius: 4px;

    code {
      flex: 1;
      font-family: monospace;
      color: #212529;
    }

    .copy-button {
      padding: 0.5rem 1rem;
      background-color: #4dabf7;
      color: white;
      border: none;
      border-radius: 4px;
      cursor: pointer;
      transition: background-color 0.2s;

      &:hover {
        background-color: #339af0;
      }
    }
  }
}
</style>