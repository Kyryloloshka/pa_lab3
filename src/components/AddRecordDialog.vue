<script setup lang="ts">
import {ref} from 'vue';
import {
  TransitionRoot,
  TransitionChild,
  Dialog,
  DialogPanel,
  DialogTitle,
} from '@headlessui/vue';
import { addRecordApi, loadAllRecordsApi } from '../services/api';

const emit = defineEmits(['update:records']);

const key = ref<number | null>(null);
const data = ref<string>('');
const isOpen = ref(false);

async function addRecord() {
  await addRecordApi(key, data);
  const newRecords = await loadAllRecordsApi();
  emit('update:records', newRecords);
}

function closeModal() {
  isOpen.value = false;
}
function openModal() {
  isOpen.value = true;
}

async function submitModal() {
  if (key.value === null || data.value === '') {
    return;
  }
  await addRecord();
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
        Add Record
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
                Add Record
              </DialogTitle>
              <div class="mt-2">
                <p class="text-sm text-gray-400">
                  Add a new record to the database.
                </p>
              </div>
              <div class="mt-2 flex flex-col gap-2">
                <input v-model.number="key" type="number" placeholder="Key (Integer)" />
                <input v-model="data" type="text" placeholder="Data" />
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
