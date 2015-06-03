<script setup lang="ts">
import {useFluent} from 'fluent-vue'

const {$t} = useFluent()
</script>

<template>
  <div class="search-bar">
    <input type="text" :placeholder="$t('search-placeholder')" class="search-input"/>
    <button class="search-button">{{ $t('search') }}</button>
  </div>
</template>

<style lang="scss" scoped>
.search-bar {
  display: flex;
  gap: 0.5rem;
  flex: 1;
  max-width: 600px;
  margin: 0 auto;

  .search-input {
    flex: 1;
    padding: 0.75rem 1rem;
    border: 2px solid #e9ecef;
    border-radius: 4px;
    font-size: 1rem;
    transition: all 0.2s;

    &:focus {
      outline: none;
      border-color: #4dabf7;
      box-shadow: 0 2px 4px rgba(77, 171, 247, 0.1);
    }
  }

  .search-button {
    padding: 0.75rem 1.5rem;
    background-color: #4dabf7;
    color: white;
    border: none;
    border-radius: 4px;
    cursor: pointer;
    transition: background-color 0.2s;
    font-weight: 500;

    &:hover {
      background-color: #339af0;
    }
  }
}
</style>