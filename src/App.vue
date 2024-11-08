<script setup lang="ts">
import { onMounted, ref } from "vue";
import AddRecordDialog from "./components/AddRecordDialog.vue";
import type { Record } from "./types";
import { loadAllRecordsApi, findRecordApi, updateRecordApi, deleteRecordApi } from "./services/api.ts";

const key = ref<number | null>(null);
const data = ref<string>("");
const record = ref<Record | null>(null);
const records = ref<Record[]>([]);

function clearInputs() {
  key.value = null;
  data.value = "";
  record.value = null;
}

onMounted(() => {
  loadAllRecordsApi(records);
});

async function findRecord() {
  await findRecordApi(key, record);
}

async function deleteRecord() {
  await deleteRecordApi(key);
  await loadAllRecordsApi(records);
  clearInputs();
}

async function updateRecord() {
  if (key.value !== null) {
    await updateRecordApi(key, { key: key.value, data: data.value });
    await loadAllRecordsApi(records);
    clearInputs();
  } else {
    alert("Please provide both key and new data.");
  }
}
function setRecords(newRecords: Record[]) {
  records.value = newRecords;
}
</script>

<template>
  <main class="flex flex-col p-5 gap-10 justify-center items-center">
    <h1 class="text-3xl">Database Manager</h1>
    <div class="flex gap-6">
      <div class="flex flex-col gap-3">
        <input v-model.number="key" type="number" placeholder="Key (Integer)" />
        <input v-model="data" type="text" placeholder="Data" />
        <AddRecordDialog :setRecords="setRecords"/>
        <button @click="findRecord">Find Record</button>
        <button @click="deleteRecord">Delete Record</button>
        <button @click="updateRecord">Update Record</button>
      </div>
      <div>
        <h2>All Records</h2>
        <ul class="flex gap-3 flex-col">
          <li v-for="record in records" :key="record.key">
            <p>Key: {{ record.key }} Data: {{ record.data }}</p>
          </li>
        </ul>
      </div>
    </div>

    <div v-if="record" class="record">
      <h2>Found Record:</h2>
      <p><strong>Key:</strong> {{ record.key }}</p>
      <p><strong>Data:</strong> {{ record.data }}</p>
    </div>
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