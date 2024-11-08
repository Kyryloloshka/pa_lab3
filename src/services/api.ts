import {invoke} from "@tauri-apps/api/core";
import { Record } from "../types";
import { Ref } from "vue";

export async function loadAllRecordsApi(records?: Ref<Record[]>): Promise<Record[] | undefined> {
    try {
        const receivedRecords = await invoke<Record[]>("get_all_records");
        if (records) {
            records.value = receivedRecords;
        }
        return receivedRecords;
    } catch (error) {
        console.error("Error loading records:", error);
    }
}

export async function addRecordApi(key: Ref<number | null>, data: Ref<string | null>) {
    if (key.value !== null && data) {
        try {
            await invoke("add_record", { key: key.value, data: data.value });
        } catch (error) {
            console.error("Error adding record:", error);
        }
    } else {
        alert("Please provide both key and new data.");
    }
}

export async function findRecordApi(key: Ref<number | null>, record: Ref<Record | null>) {
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

export async function deleteRecordApi(key: Ref<number | null>) {
    if (key.value !== null) {
        try {
            await invoke("delete_record", { key: key.value });
        } catch (error) {
            console.error("Error deleting record:", error);
        }
    } else {
        alert("Please provide a key to delete.");
    }
}

export async function updateRecordApi(key: Ref<number | null>, data: Record | null) {
    if (key.value !== null && data) {
        try {
            await invoke("update_record", { key: key.value, newData: data });
        } catch (error) {
            console.error("Error updating record:", error);
        }
    } else {
        alert("Please provide both key and new data.");
    }
}