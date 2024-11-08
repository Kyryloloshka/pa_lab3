<script setup lang="ts">

import {ref, defineProps} from 'vue';
import {
  TransitionRoot,
  TransitionChild,
  Dialog,
  DialogPanel,
  DialogTitle,
} from '@headlessui/vue';
import { updateRecordApi, loadAllRecordsApi } from '../services/api';

// Оголошення пропсів і емісії
const props = defineProps({
  setRecords: {
    type: Function,
    required: true,
  },
});

// Створення ref в дочірньому компоненті
const key = ref<number | null>(null);
const data = ref<string>('');
const isOpen = ref(false);

async function updateRecord() {
  if (key.value === null || data.value === '') {
    return;
  }
  await updateRecordApi({key: key.value, data: data.value});
  const newRecords = await loadAllRecordsApi();
  props.setRecords(newRecords);
}

function closeModal() {
  isOpen.value = false;
}
function openModal() {
  isOpen.value = true;
}

async function submitModal() {
  await updateRecord();
  key.value = null;
  data.value = '';
  closeModal();
}
</script>

<template>
  <button
      type="button"
      @click="openModal"
  >
    Update record
  </button>

  <TransitionRoot appear :show="isOpen" as="template">
    <Dialog as="div" @close="closeModal" class="relative z-10">
      <TransitionChild
          as="template"
          enter="duration-300 ease-out"
          enter-from="opacity-0"
          enter-to="opacity-100"
          leave="duration-200 ease-in"
          leave-from="opacity-100"
          leave-to="opacity-0"
      >
        <div class="fixed inset-0 bg-black/25" />
      </TransitionChild>

      <div class="fixed inset-0 overflow-y-auto">
        <div class="flex min-h-full items-center justify-center p-4 text-center">
          <TransitionChild
              as="template"
              enter="duration-300 ease-out"
              enter-from="opacity-0 scale-95"
              enter-to="opacity-100 scale-100"
              leave="duration-200 ease-in"
              leave-from="opacity-100 scale-100"
              leave-to="opacity-0 scale-95"
          >
            <DialogPanel class="w-full max-w-md transform overflow-hidden rounded-xl bg-[#2f2f2f] p-6 text-left align-middle shadow-xl transition-all">
              <DialogTitle as="h3" class="text-lg font-medium leading-6 text-gray-300">
                Update Record
              </DialogTitle>
              <div class="mt-2">
                <p class="text-sm text-gray-400">
                  Please provide both key and new data.
                </p>
              </div>
              <div class="mt-2 flex flex-col gap-2">
                <input v-model.number="key" type="number" placeholder="Key (Integer)" />
                <input v-model="data" type="text" placeholder="New data" />
              </div>
              <div class="mt-4 flex justify-between">
                <button
                    type="button"
                    class="bg-gray-700 active:bg-gray-600"
                    @click="closeModal"
                >
                  Close
                </button>
                <button
                    type="button"
                    class="bg-green-700 active:bg-green-600"
                    @click="submitModal"
                >
                  Submit
                </button>
              </div>
            </DialogPanel>
          </TransitionChild>
        </div>
      </div>
    </Dialog>
  </TransitionRoot>
</template>

<style scoped>

</style>