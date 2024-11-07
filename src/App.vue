<script setup lang="ts">
import { ref, onMounted } from "vue";
import { invoke} from "@tauri-apps/api/core";

interface Record {
  key: number;
  data: string;
}

const key = ref<number | null>(null);
const data = ref<string>('');
const record = ref<Record | null>(null);
const records = ref<Record[]>([{key: 0, data: 'test'}]);

async function addRecord() {
  if (key.value !== null && data.value) {
    try {
      await invoke("add_record", { key: key.value, data: data.value });
      await loadAllRecords(); // Перезавантажити всі записи
      clearInputs();
    } catch (error) {
      console.error("Error adding record:", error);
    }
  } else {
    alert("Please provide both key and data.");
  }
}

async function findRecord() {
  if (key.value !== null) {
    try {
      record.value = await invoke("find_record", { key: key.value });
      if (!record.value) {
      }
    } catch (error) {
      console.error("Error finding record:", error);
    }
  } else {
    alert("Please provide a key to search.");
  }
}

async function deleteRecord() {
  if (key.value !== null) {
    try {
      await invoke("delete_record", { key: key.value });
      await loadAllRecords(); // Перезавантажити всі записи
      clearInputs();
    } catch (error) {
      console.error("Error deleting record:", error);
    }
  } else {
    alert("Please provide a key to delete.");
  }
}

async function updateRecord() {
  if (key.value !== null && data.value) {
    try {
      await invoke("update_record", { key: key.value, new_data: data.value });
      await loadAllRecords(); // Перезавантажити всі записи
      clearInputs();
    } catch (error) {
      console.error("Error updating record:", error);
    }
  } else {
    alert("Please provide both key and new data.");
  }
}

async function loadAllRecords() {
  try {
    const allRecords = await invoke<Record[]>("get_all_records");
    records.value = allRecords;
  } catch (error) {
    console.error("Error loading records:", error);
  }
}

function clearInputs() {
  key.value = null;
  data.value = '';
  record.value = null;
}

onMounted(() => {
  loadAllRecords();
});
</script>

<template>
  <main class="container">
    <h1 class="text-7xl text-red-500">Database Manager</h1>
    <div class="form">
      <input v-model.number="key" type="number" placeholder="Key (Integer)" />
      <input v-model="data" type="text" placeholder="Data" />
      <button @click="addRecord">Add Record</button>
      <button @click="findRecord">Find Record</button>
      <button @click="deleteRecord">Delete Record</button>
      <button @click="updateRecord">Update Record</button>
    </div>
    <div v-if="record" class="record">
      <h2>Found Record:</h2>
      <p><strong>Key:</strong> {{ record.key }}</p>
      <p><strong>Data:</strong> {{ record.data }}</p>
    </div>

    <h2>All Records</h2>
    <ul>
      <li v-for="record in records" :key="record.key">
        <p><strong>Key:</strong> {{ record.key }}</p>
        <p><strong>Data:</strong> {{ record.data }}</p>
      </li>
    </ul>
  </main>
</template>

<style>
:root {
  font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
  font-size: 16px;
  line-height: 24px;
  font-weight: 400;

  color: #0f0f0f;
  background-color: #f6f6f6;

  font-synthesis: none;
  text-rendering: optimizeLegibility;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  -webkit-text-size-adjust: 100%;
}

.container {
  margin: 0;
  padding-top: 10vh;
  display: flex;
  flex-direction: column;
  justify-content: center;
  text-align: center;
}


a {
  font-weight: 500;
  color: #646cff;
  text-decoration: inherit;
}

a:hover {
  color: #535bf2;
}

h1 {
  text-align: center;
}

input,
button {
  border-radius: 8px;
  border: 1px solid transparent;
  padding: 0.6em 1.2em;
  font-size: 1em;
  font-weight: 500;
  font-family: inherit;
  color: #0f0f0f;
  background-color: #ffffff;
  transition: border-color 0.25s;
  box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
}

button {
  cursor: pointer;
}

button:hover {
  border-color: #396cd8;
}
button:active {
  border-color: #396cd8;
  background-color: #e8e8e8;
}

input,
button {
  outline: none;
}

#greet-input {
  margin-right: 5px;
}

@media (prefers-color-scheme: dark) {
  :root {
    color: #f6f6f6;
    background-color: #2f2f2f;
  }

  a:hover {
    color: #24c8db;
  }

  input,
  button {
    color: #ffffff;
    background-color: #0f0f0f98;
  }
  button:active {
    background-color: #0f0f0f69;
  }
}

</style>