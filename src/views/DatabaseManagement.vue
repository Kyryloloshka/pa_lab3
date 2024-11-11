<script setup lang="ts">
import { onMounted, ref } from "vue";
import AddRecordDialog from "../components/AddRecordDialog.vue";
import type { Record } from "../types";
import { loadAllRecordsApi } from "../services/api.ts";
import UpdateRecordDialog from "../components/UpdateRecordDialog.vue";
import DatabaseTable from "../components/DatabaseTable.vue";
import FindRecordDialog from "../components/FindRecordDialog.vue";
import DeleteRecordDialog from "../components/DeleteRecordDialog.vue";
import {useRoute} from "vue-router";

const route = useRoute();
const dbName = route.query.db_name as string;
const records = ref<Record[]>([]);

onMounted(() => {
  if (dbName) {
    loadAllRecordsApi(records);
  }
});

</script>

<template>
  <main class="flex flex-col p-5 gap-10 justify-center items-center">
    <div class="body__border"></div>
    <h1 class="text-3xl fade-header font-semibold tracking-wider">Database Manager</h1>
    <router-link to="/">
      Go to home page
    </router-link>
    <div class="flex gap-6 w-full">
      <DatabaseTable :records="records"/>

      <div class="flex flex-col gap-3">
        <AddRecordDialog v-model:records="records"/>
        <FindRecordDialog />
        <DeleteRecordDialog v-model:records="records"/>
        <UpdateRecordDialog v-model:records="records"/>
      </div>
    </div>
  </main>
</template>

<style scoped>
.fade-header {
  background: -webkit-linear-gradient(0deg, #aaf, #3a338b40);
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
}
</style>