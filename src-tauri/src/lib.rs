use serde_json::{from_value, json};
use std::{cmp::Ordering, time::SystemTime};

use tauri::{async_runtime::Mutex, AppHandle};
use tauri::{Manager, RunEvent, State};
use tauri_plugin_store::StoreExt;

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
async fn boot(state: State<'_, AppState>, app: AppHandle) -> Result<Vec<Todo>, ()> {
    let store = match app.store("db.json") {
        Ok(s) => s,
        Err(_) => {
            let s = app.store("db.json").map_err(|_| ())?;
            s.set("todos", json!([]));
            s.save().map_err(|_| ())?;
            s
        }
    };

    let raw = store.get("todos");

    let todos: Vec<Todo> = match raw {
        Some(value) => from_value(value).unwrap_or_default(),
        None => Vec::new(),
    };

    let mut guard = state.todos.lock().await;
    *guard = todos.clone();

    Ok(todos)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::new().build())
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
        .build(tauri::generate_context!())
        .expect("error while running tauri application")
        .run(move |app, _event| match &_event {
            RunEvent::ExitRequested { .. } => {
                println!("exiting window...");
                if let (Some(state), Ok(store)) =
                    (app.try_state::<AppState>(), app.store("db.json"))
                {
                    tauri::async_runtime::block_on(async {
                        let guard = state.todos.lock().await;
                        store.set("todos", json!(*guard));

                        if let Err(e) = store.save() {
                            eprintln!("Failed to save store: {e:?}");
                        } else {
                            println!("Todos saved successfully");
                        }
                    });
                } else {
                    eprintln!("Could not access app state or store");
                }
            }
            _ => (),
        });
}
