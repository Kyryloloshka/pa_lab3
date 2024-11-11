<template>
  <div class="create-database">
    <h2>Create a New Database</h2>
    <form @submit.prevent="submitForm">
      <div>
        <label for="dbName">Database Name:</label>
        <input v-model="dbName" type="text" id="dbName" required />
      </div>

      <div>
        <label>Fields:</label>
        <div v-for="(_, index) in fields" :key="index" class="field-input">
          <input
              v-model="fields[index]"
              type="text"
              :placeholder="`Field ${index + 1}`"
              required
          />
          <button type="button" @click="removeField(index)">Remove</button>
        </div>
        <button type="button" @click="addField">Add Field</button>
      </div>

      <button type="submit">Create Database</button>
    </form>
  </div>
</template>

<script setup lang="ts">
import { ref } from "vue";
import {createDb} from "../services/api.js";
const dbName = ref<string>("");
const fields = ref<string[]>(['key']) // Initial field 'key' as required by your setup
function addField() {
  fields.value.push(""); // Add a new field input
}
function removeField(index: number) {
  if (fields.value.length > 1) fields.value.splice(index, 1);
}
async function submitForm() {
  await createDb(dbName.value, fields.value);
  dbName.value = "";
  fields.value = ["key"];
}
</script>

<style scoped>
.create-database {
  max-width: 400px;
  margin: auto;
}

.field-input {
  display: flex;
  align-items: center;
  gap: 8px;
}

button {
  margin-top: 10px;
}
</style>
