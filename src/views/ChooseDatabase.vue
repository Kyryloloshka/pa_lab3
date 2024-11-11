<script setup lang="ts">
import { ref, onMounted } from "vue";
import { getDatabases, loadDatabase, loadAllRecordsApi } from "../services/api";
import { Record } from "../types";
import router from "../router";

// Стан для збереження списку баз даних, обраної бази та записів
const databases = ref<any[]>([]);
const selectedDatabase = ref<string>("");
const records = ref<Record[]>([]);

// Завантажити всі бази даних при монтуванні компонента
onMounted(async () => {
  await loadDatabases();
});

// Завантажити бази даних
async function loadDatabases() {
  try {
    databases.value = await getDatabases();
    selectedDatabase.value = ""; // Скидаємо вибір бази
    records.value = []; // Очищаємо записи
  } catch (error) {
    console.error("Error loading databases:", error);
  }
}

// Завантажити записи з обраної бази даних
async function loadChosenDatabase(name: string) {
  try {
    await loadDatabase(name); // Завантажуємо вибрану базу через API
    selectedDatabase.value = name; // Оновлюємо вибрану базу
    await loadAllRecords(name); // Завантажуємо всі записи цієї бази
    await router.push({ name: "manage_db", params: { db_name: name } });
  } catch (error) {
    console.error("Error loading database:", error);
  }
}

// Завантажити всі записи для обраної бази даних
async function loadAllRecords(name: string) {
  try {
    const allRecords = await loadAllRecordsApi(records, name);
    if (allRecords) {
      records.value = allRecords;
    }
  } catch (error) {
    console.error("Error loading records:", error);
  }
}
</script>

<template>
  <div>
    <h1>Manage Databases</h1>

    <div class="p-12" v-if="databases.length">
      <h2 class="tet-2xl mb-6">Available Databases</h2>
      <ul class="flex flex-col gap-3">
        <li v-for="db in databases" :key="db.name">
          <button @click="loadChosenDatabase(db.name)">
            {{ db.name }}
          </button>
        </li>
      </ul>
    </div>
  </div>
</template>
