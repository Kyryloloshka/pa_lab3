<script setup lang="ts">

import {computed, defineProps, ref} from "vue";
const props = defineProps({
  records: {
    type: Array,
    required: true,
  },
});
const page = ref(1);
const itemsPerPage = ref(5);
const pageCount = computed(() => Math.ceil(props.records.length / itemsPerPage.value));
</script>

<template>
  <v-data-table
      theme="dark"
      v-model:page="page"
      class="rounded-lg"
      :items="props.records"
      :items-per-page="itemsPerPage"
  >
    <template v-slot:top>
      <v-text-field
          :model-value="itemsPerPage"
          class="pa-2"
          label="Items per page"
          max="15"
          min="0"
          type="number"
          hide-details
          @update:model-value="itemsPerPage = parseInt($event, 10)"
      ></v-text-field>
    </template>

    <template v-slot:bottom>
      <div class="text-center pt-2">
        <v-pagination
            v-model="page"
            :length="pageCount"
        ></v-pagination>
      </div>
    </template>
  </v-data-table>
</template>

<style scoped>

</style>