#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::fs::File;
use std::time::SystemTime;
use serde_json;
use serde::{ Serialize, Deserialize };

#[derive(Serialize, Deserialize, Debug)]
struct Task {
    content: String,
    created: u64
}

#[tauri::command]
fn new_task(name: &str) {
    let file = File::open(r#"C:\Users\MS1\Desktop\tasks.json"#).unwrap();
    let mut tasks: Vec<Task> = serde_json::from_reader(&file).unwrap();
    println!("Prior to modification: {:?}", tasks);
    let task = Task {
        content: String::from(name),
        created: SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).expect("...").as_secs()
    };
    tasks.push(task);
    println!("Post modification: {:?}", tasks);

    let file = File::create(r#"C:\Users\MS1\Desktop\tasks.json"#).unwrap();
    serde_json::to_writer(&file, &tasks).expect("...");
}

#[tauri::command]
fn get_tasks() -> String {
    let data = std::fs::read_to_string(r#"C:\Users\MS1\Desktop\tasks.json"#).unwrap();
    
    println!("{}", data);

    return data;
}

#[tauri::command]
fn delete_task(index: usize) {
    let file = File::open(r#"C:\Users\MS1\Desktop\tasks.json"#).unwrap();
    let mut tasks: Vec<Task> = serde_json::from_reader(&file).unwrap();
    tasks.remove(index);

    let file = File::create(r#"C:\Users\MS1\Desktop\tasks.json"#).unwrap();
    serde_json::to_writer(&file, &tasks).expect("...");
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![new_task, get_tasks, delete_task])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
