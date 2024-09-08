// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod entities;
mod migration;

use core::panic;
use migration::Migrator;
use sea_orm::{Database, DatabaseConnection, DbErr};
use sea_orm_migration::MigratorTrait;
use std::{env, fs};
use tauri::{async_runtime::block_on, Manager};
use window_shadows::set_shadow;

#[derive(Clone)]
struct AppState {
    conn: DatabaseConnection,
}

fn main() {
    let state = match block_on(run()) {
        Ok(conn) => AppState { conn },
        Err(err) => panic!("{}", err),
    };

    tauri::Builder::default()
        .setup(|app| {
            app.get_window("main")
                .and_then(|window| Some(set_shadow(window, true)));
            Ok(())
        })
        .manage(state)
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
    //.invoke_handler(tauri::generate_handler![greet])
}

async fn run() -> Result<DatabaseConnection, DbErr> {
    let db_url = "sqlite://database.sqlite?mode=rwc";
    let conn = Database::connect(db_url)
        .await
        .expect("Database connection failed");
    Migrator::up(&conn, None).await.unwrap();
    Ok(conn)
}
