// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{
    fs::{self, File},
    io::Write,
};

use serde_json::json;
use tauri::{Manager, Wry};
use tauri_plugin_store::{with_store, StoreBuilder, StoreCollection};

use tauri::{command, ipc::Channel, AppHandle, Runtime, Window};

#[command]
async fn set_refresh_token(
    app: AppHandle<Wry>,
    token: String,
) -> Result<(), tauri_plugin_store::Error> {
    // Access the store collection
    let stores = app.try_state::<StoreCollection<Wry>>().expect("stores");
    // Define the path for the store (adjust as needed)
    let path = app
        .path()
        .app_data_dir()
        .expect("unable to find data dir")
        .join("data.json");

    // Use the store with the `with_store` helper
    with_store(app.clone(), stores, path, |store| {
        // Retrieve a value from the store
        store.insert("refresh_token".to_string(), json!(token))?;
        store.save()?;
        // Return an owned String by cloning the value
        Ok(())
    })
}

#[command]
async fn get_refresh_token(app: AppHandle<Wry>) -> Option<String> {
    let stores = app.try_state::<StoreCollection<Wry>>().expect("stores");
    let path = app
        .path()
        .app_data_dir()
        .expect("unable to find data dir")
        .join("data.json");

    with_store(app.clone(), stores, path, |store| {
        store
            .get("refresh_token")
            .and_then(|s| s.as_str().map(|s| s.to_owned())) // Directly map to the owned String
            .ok_or_else(|| {
                tauri_plugin_store::Error::Tauri(tauri::Error::AssetNotFound("test.".to_string()))
            })
        // Convert to expected error type
    })
    .ok() // Convert the result into `Option`
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::default().build())
        .setup(|app| {
            let path = app
                .path()
                .app_data_dir()
                .expect("unable to find data dir")
                .join("data.json");

            if !path.exists() {
                if let Some(parent) = path.parent() {
                    fs::create_dir_all(parent)?;
                }

                let mut file = fs::File::create(&path)?;
                file.write_all(b"{}")?;
            }

            let stores = app
                .handle()
                .try_state::<StoreCollection<Wry>>()
                .ok_or("Store not found")?;

            with_store(app.handle().clone(), stores, path, |store| {
                // Note that values must be serde_json::Value instances,
                // otherwise, they will not be compatible with the JavaScript bindings.
                store.insert("some-key".to_string(), json!({ "value": 5 }))?;

                // Get a value from the store.
                let value = store
                    .get("some-key")
                    .expect("Failed to get value from store");
                println!("{}", value); // {"value":5}

                // You can manually save the store after making changes.
                // Otherwise, it will save upon graceful exit as described above.
                store.save()?;

                Ok(())
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_refresh_token,
            set_refresh_token
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
