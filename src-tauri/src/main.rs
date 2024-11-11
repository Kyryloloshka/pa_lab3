use serde::{Deserialize, Serialize};
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Seek, SeekFrom, Write};
use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};
use tauri::State;

#[derive(Serialize, Deserialize, Clone, Debug)]
struct Record {
    key: i32,
    data: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct DatabaseMetadata {
    name: String,
    fields: Vec<String>,
}

#[derive(Debug)]
struct Database {
    name: String,
    index: BTreeMap<i32, u64>,
    data_file: File,
    fields: Vec<String>,
}

impl Database {
    fn new(name: &str, fields: Vec<String>) -> Self {
        let file_path = format!("{}.db", name);
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(&file_path)
            .unwrap();

        let mut db = Self {
            name: name.to_string(),
            index: BTreeMap::new(),
            data_file: file,
            fields,
        };

        db.load_index();
        db
    }

    fn load_index(&mut self) {
        let mut reader = BufReader::new(&self.data_file);
        let mut position = 0;

        loop {
            let mut buffer = String::new();
            let bytes_read = reader.read_line(&mut buffer).unwrap();
            if bytes_read == 0 { break; } // End of file

            if let Ok(record) = serde_json::from_str::<Record>(&buffer.trim()) {
                self.index.insert(record.key, position);
            }

            position += bytes_read as u64;
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

    fn get_all_records(&mut self) -> Vec<Record> {
        let mut records = Vec::new();
        self.data_file.seek(SeekFrom::Start(0)).unwrap();
        let reader = BufReader::new(&self.data_file);

        for line in reader.lines() {
            if let Ok(record) = serde_json::from_str::<Record>(&line.unwrap().trim()) {
                records.push(record);
            }
        }
        records
    }
}

// Структура для збереження кількох баз даних
struct DatabaseManager {
    databases: BTreeMap<String, Database>,
}

impl DatabaseManager {
    fn new() -> Self {
        Self {
            databases: BTreeMap::new(),
        }
    }

    fn create_database(&mut self, name: String, fields: Vec<String>) {
        let db = Database::new(&name, fields);
        self.databases.insert(name.clone(), db);
    }

    fn load_database(&mut self, name: &str) -> Option<&mut Database> {
        self.databases.get_mut(name)
    }

    fn get_all_databases(&self) -> Vec<DatabaseMetadata> {
        self.databases.iter().map(|(name, db)| {
            DatabaseMetadata {
                name: name.clone(),
                fields: db.fields.clone(),
            }
        }).collect()
    }
}

#[tauri::command]
fn create_database(name: String, fields: Vec<String>, state: State<Mutex<DatabaseManager>>) {
    let mut manager = state.lock().unwrap();
    manager.create_database(name, fields);
}

#[tauri::command]
fn get_databases(state: State<Mutex<DatabaseManager>>) -> Vec<DatabaseMetadata> {
    let manager = state.lock().unwrap();
    manager.get_all_databases()
}

#[tauri::command]
fn load_database(name: String, state: State<Mutex<DatabaseManager>>) -> Option<Vec<Record>> {
    let mut manager = state.lock().unwrap();
    if let Some(db) = manager.load_database(&name) {
        Some(db.get_all_records())
    } else {
        None
    }
}

#[tauri::command]
fn add_record(name: String, key: i32, data: String, state: State<Mutex<DatabaseManager>>) {
    let mut manager = state.lock().unwrap();
    if let Some(db) = manager.load_database(&name) {
        db.add_record(key, data);
    }
}

#[tauri::command]
fn get_all_records(name: String, state: State<Mutex<DatabaseManager>>) -> Option<Vec<Record>> {
    let mut manager = state.lock().unwrap();
    if let Some(db) = manager.load_database(&name) {
        Some(db.get_all_records())
    } else {
        None
    }
}

fn main() {
    tauri::Builder::default()
        .manage(Mutex::new(DatabaseManager::new()))
        .invoke_handler(tauri::generate_handler![
            create_database,
            get_databases,
            load_database,
            add_record,
            get_all_records
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
