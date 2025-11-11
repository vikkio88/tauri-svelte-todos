use std::{cmp::Ordering, time::SystemTime};
use tauri::async_runtime::Mutex;
use tauri::State;

mod models;
use models::{AppState, Todo};

pub fn sort_todos(todos: &mut Vec<Todo>) {
    todos.sort_by(|a, b| match (a.done, b.done) {
        (false, true) => Ordering::Less,
        (true, false) => Ordering::Greater,
        _ => b.updated.cmp(&a.updated),
    });
}

#[tauri::command]
async fn add_todo(description: &str, state: State<'_, AppState>) -> Result<Vec<Todo>, ()> {
    let mut todos = state.todos.lock().await;
    let now = SystemTime::now();

    todos.push(Todo {
        description: description.to_string(),
        done: false,
        created: now,
        updated: now,
    });

    sort_todos(&mut todos);
    Ok(todos.to_vec())
}

#[tauri::command]
async fn update_todo(idx: usize, done: bool, state: State<'_, AppState>) -> Result<Vec<Todo>, ()> {
    let mut todos = state.todos.lock().await;

    if idx >= todos.len() {
        return Ok(todos.to_vec());
    }

    if let Some(todo) = todos.get_mut(idx) {
        todo.done = done;
        todo.updated = SystemTime::now();
    } else {
        return Ok(todos.to_vec());
    }

    sort_todos(&mut todos);
    Ok(todos.to_vec())
}

#[tauri::command]
async fn remove_todo(idx: i32, state: State<'_, AppState>) -> Result<Vec<Todo>, ()> {
    let mut todos = state.todos.lock().await;
    if idx < 0 || idx as usize > todos.len() {
        return Ok(todos.to_vec());
    }

    todos.remove(idx as usize);
    sort_todos(&mut todos);
    Ok(todos.to_vec())
}

#[tauri::command]
async fn boot(state: State<'_, AppState>) -> Result<Vec<Todo>, ()> {
    let todos = state.todos.lock().await;
    Ok(todos.to_vec())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(AppState {
            todos: Mutex::new(Vec::new()),
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            boot,
            add_todo,
            remove_todo,
            update_todo
        ])
        //https://github.com/tauri-apps/tauri/discussions/10531#discussioncomment-10274882
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
