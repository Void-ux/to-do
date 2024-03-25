use rusqlite::Connection;
use tauri::AppHandle;
use std::fs;

pub fn initialise_database(app_handle: &AppHandle) -> Result<Connection, rusqlite::Error> {
    println!("Creating database at {}", app_handle.path_resolver().app_dir().unwrap().to_str().unwrap());
    let app_dir = app_handle.path_resolver().app_dir().expect("The app data directory wasn't identified.");
    fs::create_dir_all(&app_dir).expect("Creating the app directory failed.");
    let sqlite_path = app_dir.join("todos.sqlite");

    let conn = Connection::open(sqlite_path)?;
    Ok(conn)
}
