use serde::{Deserialize, Serialize};
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Seek, SeekFrom, Write};
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

        let mut db = Self {
            index: BTreeMap::new(),
            data_file: file,
        };

        db.load_index();  // Завантаження індексу при створенні
        db
    }

    // Завантаження індексу з файлу
    fn load_index(&mut self) {
        let mut reader = BufReader::new(&self.data_file);
        let mut position = 0;

        loop {
            let mut buffer = String::new();
            let bytes_read = reader.read_line(&mut buffer).unwrap();
            if bytes_read == 0 { break; }  // Кінець файлу

            if let Ok(record) = serde_json::from_str::<Record>(&buffer.trim()) {
                self.index.insert(record.key, position);
            }

            position += bytes_read as u64;
        }
    }

    fn add_record(&mut self, key: i32, data: String) {
        // Перевірка на існування ключа
        if self.index.contains_key(&key) {
            println!("Key already exists");
            return;
        }

        let position = self.data_file.seek(SeekFrom::End(0)).unwrap();
        let record = Record { key, data };

        // Запис нового запису у файл
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
            let mut reader = BufReader::new(&self.data_file);
            reader.read_line(&mut buffer).unwrap();
            return serde_json::from_str(&buffer.trim()).ok();
        }
        None
    }

    fn delete_record(&mut self, key: i32) -> Option<Record> {
        if let Some(&position) = self.index.get(&key) {
            if let Some(record) = self.find_record(key) {
                self.index.remove(&key);
                self.data_file.seek(SeekFrom::Start(position)).unwrap();
                self.data_file.write_all(b"{\"deleted\":true}\n").unwrap();
                println!("Record marked as deleted");
                return Some(record);
            }
        }
        println!("Key not found");
        None
    }

    fn update_record(&mut self, key: i32, new_data: String) {
        if let Some(&position) = self.index.get(&key) {
            self.data_file.seek(SeekFrom::Start(position)).unwrap();
            let updated_record = Record { key, data: new_data };
            let serialized = serde_json::to_string(&updated_record).unwrap();

            // Проконтролюємо, щоб новий запис перекривав старий повністю.
            // Якщо новий запис коротший, заповнюємо залишок пробілами.
            let record_with_padding = format!("{:<width$}\n", serialized, width = serialized.len().max(100));

            self.data_file.write_all(record_with_padding.as_bytes()).unwrap();
            println!("Record updated");
        } else {
            println!("Key not found");
        }
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
        .manage(Mutex::new(Database::new()))
        .invoke_handler(tauri::generate_handler![add_record, find_record, delete_record, update_record, get_all_records])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
