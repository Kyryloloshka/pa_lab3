import {invoke} from "@tauri-apps/api/core";
import {Ref} from "vue";
import {DatabaseMetadata, Record} from "../types";

// Завантаження всіх записів з вибраної бази даних
export async function loadAllRecordsApi(records: Ref<Record[]>, db_name: string): Promise<Record[] | undefined> {
    try {
        const receivedRecords = await invoke<Record[]>("get_all_records", { name: db_name });
        if (receivedRecords) {
            records.value = receivedRecords;
        }
        return receivedRecords;
    } catch (error) {
        console.error("Error loading records:", error);
    }
}

// Додавання нового запису до вибраної бази даних
export async function addRecordApi(db_name: string, key: Ref<number | null>, data: Ref<string | null>) {
    if (key.value !== null && data.value) {
        try {
            await invoke("add_record", { name: db_name, key: key.value, data: data.value });
        } catch (error) {
            console.error("Error adding record:", error);
        }
    } else {
        alert("Please provide both key and new data.");
    }
}

// Пошук запису за ключем у вибраній базі даних
export async function findRecordApi(db_name: string, key: Ref<number | null>, record: Ref<Record | null>) {
    if (key.value !== null) {
        try {
            record.value = await invoke("find_record", {name: db_name, key: key.value});
        } catch (error) {
            console.error("Error finding record:", error);
        }
    } else {
        alert("Please provide a key to search.");
    }
}

// Видалення запису з вибраної бази даних за ключем
export async function deleteRecordApi(db_name: string, key: Ref<number | null>) {
    if (key.value !== null) {
        try {
            await invoke("delete_record", { name: db_name, key: key.value });
        } catch (error) {
            console.error("Error deleting record:", error);
        }
    } else {
        alert("Please provide a key to delete.");
    }
}

// Оновлення запису у вибраній базі даних
export async function updateRecordApi(db_name: string, record: Record | null) {
    if (record !== null && record.key !== null && record.data) {
        try {
            await invoke("update_record", { name: db_name, key: record.key, newData: record.data });
        } catch (error) {
            console.error("Error updating record:", error);
        }
    } else {
        alert("Please provide both key and new data.");
    }
}

// Створення нової бази даних
export async function createDb(name: string, fields: string[]) {
    try {
        await invoke("create_database", { name, fields });
        alert(`Database "${name}" created successfully!`);
    } catch (error) {
        console.error("Error creating database:", error);
    }
}

// Отримання всіх баз даних
export async function getDatabases(): Promise<DatabaseMetadata[]> {
    try {
        return await invoke("get_databases") as DatabaseMetadata[];
    } catch (error) {
        console.error("Error getting databases:", error);
        return [];
    }
}

// Завантаження вибраної бази даних
export async function loadDatabase(name: string) {
    try {
        await invoke("load_database", { name });
    } catch (error) {
        console.error("Error loading database:", error);
    }
}
