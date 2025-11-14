use serde_json::{from_value, json};
use std::{cmp::Ordering, time::SystemTime};

use tauri::{async_runtime::Mutex, AppHandle};
use tauri::{Manager, RunEvent, State};
use tauri_plugin_sql::{Migration, MigrationKind};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let migrations = vec![Migration {
        version: 1,
        description: "create_initial_tables",
        sql: r#"
            CREATE TABLE todos (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                description TEXT NOT NULL,
                done INTEGER NOT NULL DEFAULT 0,
                created INTEGER NOT NULL,
                updated INTEGER NOT NULL
            );
        "#,
        kind: MigrationKind::Up,
    }];

    tauri::Builder::default()
        .plugin(
            tauri_plugin_sql::Builder::default()
                .add_migrations("sqlite:mydatabase.db", migrations)
                .build(),
        )
        .plugin(tauri_plugin_opener::init())
        .build(tauri::generate_context!())
        .expect("error while running tauri application")
        .run(move |app, _event| match &_event {
            RunEvent::ExitRequested { .. } => {
                println!("exiting window...");
            }
            _ => (),
        });
}
