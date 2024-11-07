use serde::{Deserialize, Serialize};
use std::fs::{File, OpenOptions};
use std::io::{Read, Write, Seek, SeekFrom};
use std::collections::BTreeMap;
use std::sync::Mutex;
use tauri::State;

#[derive(Serialize, Deserialize, Clone, Debug)]
struct Record {
    key: i32,
    data: String,
}

struct Database {
    index: BTreeMap<i32, u64>,  // Індекс для зберігання позицій
    data_file: File,
}

impl Database {
    fn new() -> Self {
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open("data.db")
            .unwrap();
        Self {
            index: BTreeMap::new(),
            data_file: file,
        }
    }

    fn add_record(&mut self, key: i32, data: String) {
        if self.index.contains_key(&key) {
            println!("Key already exists");
            return;
        }
        let position = self.data_file.seek(SeekFrom::End(0)).unwrap();
        let record = Record { key, data };
        self.data_file
            .write_all(&serde_json::to_vec(&record).unwrap())
            .unwrap();
        self.data_file.write_all(b"\n").unwrap();
        self.index.insert(key, position);
    }

    fn find_record(&mut self, key: i32) -> Option<Record> {
        if let Some(&position) = self.index.get(&key) {
            self.data_file.seek(SeekFrom::Start(position)).unwrap();
            let mut buffer = String::new();
            self.data_file.read_to_string(&mut buffer).unwrap();
            return serde_json::from_str(&buffer).ok();
        }
        None
    }

    fn delete_record(&mut self, key: i32) {
        if self.index.remove(&key).is_some() {
            println!("Record marked as deleted");
        } else {
            println!("Key not found");
        }
    }

    fn update_record(&mut self, key: i32, new_data: String) {
        if let Some(&position) = self.index.get(&key) {
            self.data_file.seek(SeekFrom::Start(position)).unwrap();
            self.data_file
                .write_all(&serde_json::to_vec(&Record { key, data: new_data }).unwrap())
                .unwrap();
        } else {
            println!("Key not found");
        }
    }
    fn get_all_records(&mut self) -> Vec<Record> {
        let mut records = Vec::new();
        self.data_file.seek(SeekFrom::Start(0)).unwrap();

        let mut buffer = String::new();
        self.data_file.read_to_string(&mut buffer).unwrap();

        for line in buffer.lines() {
            if let Ok(record) = serde_json::from_str::<Record>(line) {
                records.push(record);
            }
        }
        records
    }
}

#[tauri::command]
fn add_record(db: State<Mutex<Database>>, key: i32, data: String) {
    db.lock().unwrap().add_record(key, data);
}

#[tauri::command]
fn find_record(db: State<Mutex<Database>>, key: i32) -> Option<Record> {
    db.lock().unwrap().find_record(key)
}

#[tauri::command]
fn delete_record(db: State<Mutex<Database>>, key: i32) {
    db.lock().unwrap().delete_record(key);
}

#[tauri::command]
fn update_record(db: State<Mutex<Database>>, key: i32, new_data: String) {
    db.lock().unwrap().update_record(key, new_data);
}

#[tauri::command]
fn get_all_records(db: State<Mutex<Database>>) -> Vec<Record> {
    db.lock().unwrap().get_all_records()
}

fn main() {
    tauri::Builder::default()
        .manage(Mutex::new(Database::new()))  // Додаємо Mutex для потокобезпечності
        .invoke_handler(tauri::generate_handler![add_record, find_record, delete_record, update_record, get_all_records])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
