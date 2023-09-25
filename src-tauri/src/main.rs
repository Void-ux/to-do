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

fn tasks_path() -> &'static str {
    if cfg!(windows) {
        return r#"C:\Users\MS1\Desktop\tasks.json"#;
    } else if cfg!(target_os = "macos") {
        return r#"/Users/dan/tasks.json"#;
    } else {
        return ""
    }
}


#[tauri::command]
fn new_task(name: &str) {
    let file = File::open(tasks_path()).unwrap();
    let mut tasks: Vec<Task> = serde_json::from_reader(&file).unwrap();
    println!("Prior to modification: {:?}", tasks);
    let task = Task {
        content: String::from(name),
        created: SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).expect("...").as_secs()
    };
    tasks.push(task);
    println!("Post modification: {:?}", tasks);

    let file = File::create(tasks_path()).unwrap();
    serde_json::to_writer(&file, &tasks).expect("...");
}

#[tauri::command]
fn get_tasks() -> String {
    let data = std::fs::read_to_string(tasks_path()).unwrap();
    
    println!("{}", data);

    return data;
}

#[tauri::command]
fn delete_task(index: usize) {
    let file = File::open(tasks_path()).unwrap();
    let mut tasks: Vec<Task> = serde_json::from_reader(&file).unwrap();
    tasks.remove(index);

    let file = File::create(tasks_path()).unwrap();
    serde_json::to_writer(&file, &tasks).expect("...");
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![new_task, get_tasks, delete_task])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
