<script setup lang="ts">
import {onMounted, ref} from "vue";
import AddRecordDialog from "./components/AddRecordDialog.vue";
import type { Record } from "./types";
import { loadAllRecordsApi, findRecordApi, deleteRecordApi } from "./services/api.ts";
import UpdateRecordDialog from "./components/UpdateRecordDialog.vue";
import DatabaseTable from "./components/DatabaseTable.vue";

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

function setRecords(newRecords: Record[]) {
  records.value = newRecords;
}

</script>

<template>
  <main class="flex flex-col p-5 gap-10 justify-center items-center">

    <h1 class="text-3xl">Database Manager</h1>
    <div class="flex gap-6 w-full">
      <DatabaseTable :records="records"/>

      <div class="flex flex-col gap-3">
        <input v-model.number="key" type="number" placeholder="Key (Integer)" />
        <input v-model="data" type="text" placeholder="Data" />
        <AddRecordDialog :setRecords="setRecords"/>
        <button @click="findRecord">Find Record</button>
        <button @click="deleteRecord">Delete Record</button>
        <UpdateRecordDialog :setRecords="setRecords"/>
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


</style>