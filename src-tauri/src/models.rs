use serde::{Deserialize, Serialize};
use tauri::async_runtime::Mutex;

use std::time::SystemTime;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Todo {
    pub description: String,
    pub done: bool,
    pub created: SystemTime,
    pub updated: SystemTime,
}

pub struct AppState {
    pub todos: Mutex<Vec<Todo>>,
}
