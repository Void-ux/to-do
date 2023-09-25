#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod database;
mod state;

use rusqlite::{params, types::Null};
use tauri::{ Manager, State };

use database::initialise_database;
use state::AppState;


#[tauri::command]
fn new_task(state: State<AppState>, title: &str) {
    let mut binding = state.conn.lock().unwrap();
    let conn = binding.as_mut().unwrap();

    conn.execute(
        "INSERT INTO tasks (title, description) VALUES (?1, ?2)",
        params![title, Null]
    ).unwrap();
}

#[tauri::command]
fn get_tasks(state: State<AppState>) -> Vec<(u64, String, u64)> {
    let mut binding = state.conn.lock().unwrap();
    let conn = binding.as_mut().unwrap();

    let mut statement = conn.prepare("SELECT id, title, created FROM tasks ORDER BY created").unwrap();
    let mut rows = statement.query([]).unwrap();
    let mut tasks: Vec<(u64, String, u64)> = Vec::new();

    while let Some(row) = rows.next().unwrap() {
        tasks.push((row.get("id").unwrap(), row.get("title").unwrap(), row.get("created").unwrap()))
    }

    tasks
}

#[tauri::command]
fn delete_task(state: State<AppState>, id: usize) {
    let mut binding = state.conn.lock().unwrap();
    let conn = binding.as_mut().unwrap();

    conn.execute("DELETE FROM tasks WHERE id=?1", params![id]).unwrap();
}

fn main() {
    tauri::Builder::default()
        .manage(AppState { conn: Default::default() })
        .invoke_handler(tauri::generate_handler![new_task, get_tasks, delete_task])
        .setup(|app| {
            let handle = app.handle();

            let app_state: State<AppState> = handle.state();
            let conn = initialise_database(&handle).expect("Database init failed");
            conn.execute(
                r#"CREATE TABLE IF NOT EXISTS tasks (
                    id          INTEGER PRIMARY KEY,
                    title       TEXT,
                    description TEXT,
                    created     INTEGER DEFAULT (unixepoch())
                )"#,
                []
            )?;
            *app_state.conn.lock().unwrap() = Some(conn);
            
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
