<template>
  <div class="example-component">
    <div v-if="loading">Loading...</div>
    <div v-else-if="error">{{ error }}</div>
    <div v-else>
      <h2>Example Data</h2>
      <pre>{{ data }}</pre>
      
      <div class="actions">
        <button @click="fetchData">Fetch Data</button>
        <button @click="submitData">Submit Data</button>
        <button @click="updateData">Update Data</button>
        <button @click="removeData">Delete Data</button>
      </div>
    </div>
  </div>
</template>

<script lang="ts" setup>
import { ref } from 'vue';
import { getExampleData, postExampleData, updateExampleData, deleteExampleData } from '../api/models/example-api';

const data = ref<any>(null);
const loading = ref(false);
const error = ref<string | null>(null);

const fetchData = async () => {
  loading.value = true;
  error.value = null;
  try {
    data.value = await getExampleData();
  } catch (e: any) {
    error.value = e.message || 'Failed to fetch data';
  } finally {
    loading.value = false;
  }
};

const submitData = async () => {
  loading.value = true;
  error.value = null;
  try {
    const newData = { message: 'New Data' };
    data.value = await postExampleData(newData);
  } catch (e: any) {
    error.value = e.message || 'Failed to submit data';
  } finally {
    loading.value = false;
  }
};

const updateData = async () => {
  loading.value = true;
  error.value = null;
  try {
    const updatedData = { message: 'Updated Data' };
    data.value = await updateExampleData('example-id', updatedData);
  } catch (e: any) {
    error.value = e.message || 'Failed to update data';
  } finally {
    loading.value = false;
  }
};

const removeData = async () => {
  loading.value = true;
  error.value = null;
  try {
    await deleteExampleData('example-id');
    data.value = null;
  } catch (e: any) {
    error.value = e.message || 'Failed to delete data';
  } finally {
    loading.value = false;
  }
};
</script>

<style scoped>
.example-component {
  padding: 20px;
}

.actions {
  margin-top: 20px;
}

button {
  margin-right: 10px;
  padding: 8px 16px;
  border-radius: 4px;
  border: 1px solid #ddd;
  background-color: #fff;
  cursor: pointer;
}

button:hover {
  background-color: #f5f5f5;
}
</style>