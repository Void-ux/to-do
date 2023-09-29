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
fn create_task(state: State<AppState>, title: &str, list: u64) {
    let mut binding = state.conn.lock().unwrap();
    let conn = binding.as_mut().unwrap();

    conn.execute(
        "INSERT INTO tasks (title, description, list) VALUES (?1, ?2, ?3)",
        params![title, Null, list]
    ).unwrap();
}

#[tauri::command]
fn get_tasks(state: State<AppState>, task_list_id: Option<u64>) -> Vec<(u64, String, u64)> {
    if task_list_id.is_none() {
        return Vec::new()
    }
    let mut binding = state.conn.lock().unwrap();
    let conn = binding.as_mut().unwrap();

    let mut statement = conn.prepare(
        "SELECT id, title, created FROM tasks WHERE list = ?1 ORDER BY created"
    ).unwrap();
    let mut rows = statement.query([Some(task_list_id)]).unwrap();
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

#[tauri::command]
fn get_task_lists(state: State<AppState>) -> Vec<(u64, String)> {
    let mut binding = state.conn.lock().unwrap();
    let conn = binding.as_mut().unwrap();

    let mut statement = conn.prepare("SELECT id, name FROM task_lists ORDER BY created").unwrap();
    let mut rows = statement.query([]).unwrap();
    let mut lists: Vec<(u64, String)> = Vec::new();

    while let Some(row) = rows.next().unwrap() {
        lists.push((row.get("id").unwrap(), row.get("name").unwrap()))
    }

    lists
}

#[tauri::command]
fn create_task_list(state: State<AppState>, name: Option<&str>) -> String {
    let mut binding = state.conn.lock().unwrap();
    let conn = binding.as_mut().unwrap();
    let mut query = conn.prepare("INSERT INTO task_lists (name) VALUES (?1)").unwrap();

    if let Some(name) = name {
        let task_name = name.to_owned();
        query.execute(params![task_name]).unwrap();
        task_name
    } else {
        let mut statement = conn.prepare("SELECT max(id) FROM task_lists").unwrap();
        let val: Option<u64> = statement.query_row([], |row| row.get(0)).unwrap();
        let task_name = format!("MyList{}", if let Some(val) = val { val } else { 0 });
        query.execute(params![task_name]).unwrap();
        task_name
    }
}

fn main() {
    tauri::Builder::default()
        .manage(AppState { conn: Default::default() })
        .invoke_handler(tauri::generate_handler![create_task, get_tasks, delete_task, get_task_lists, create_task_list])
        .setup(|app| {
            let handle = app.handle();

            let app_state: State<AppState> = handle.state();
            let conn = initialise_database(&handle).expect("Database init failed");
            conn.execute(
                r#"CREATE TABLE IF NOT EXISTS tasks (
                    id          INTEGER PRIMARY KEY,
                    title       TEXT,
                    description TEXT,
                    list        INTEGER,
                    created     INTEGER DEFAULT (unixepoch()),
                    FOREIGN KEY (list) REFERENCES task_lists(id) 
                )"#,
                []
            )?;
            conn.execute(
                r#"CREATE TABLE IF NOT EXISTS task_lists (
                    id          INTEGER PRIMARY KEY,
                    name        TEXT    UNIQUE NOT NULL,
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
