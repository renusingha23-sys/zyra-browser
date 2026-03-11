#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Mutex;
use tauri::Manager;
use url::Url;

struct BrowserState {
    history: Mutex<Vec<String>>,
    bookmarks: Mutex<Vec<String>>,
}

fn normalize_input(input: &str) -> String {
    if input.starts_with("http://") || input.starts_with("https://") {
        input.to_string()
    } else if input.contains('.') && !input.contains(' ') {
        format!("https://{}", input)
    } else {
        format!("https://www.google.com/search?q={}", input.replace(" ", "+"))
    }
}

#[tauri::command]
fn navigate(window: tauri::WebviewWindow, input: String) -> Result<String, String> {
    let url = normalize_input(&input);

    let parsed = Url::parse(&url).map_err(|e| e.to_string())?;
    window.navigate(parsed).map_err(|e| e.to_string())?;

    Ok(url)
}

#[tauri::command]
fn record_history(state: tauri::State<BrowserState>, url: String) {
    let mut history = state.history.lock().unwrap();
    history.push(url);
}

#[tauri::command]
fn get_history(state: tauri::State<BrowserState>) -> Vec<String> {
    state.history.lock().unwrap().clone()
}

#[tauri::command]
fn add_bookmark(state: tauri::State<BrowserState>, url: String) {
    let mut bookmarks = state.bookmarks.lock().unwrap();
    bookmarks.push(url);
}

#[tauri::command]
fn get_bookmarks(state: tauri::State<BrowserState>) -> Vec<String> {
    state.bookmarks.lock().unwrap().clone()
}

#[tauri::command]
fn download_file(url: String, path: String) -> Result<(), String> {
    let response = reqwest::blocking::get(url).map_err(|e| e.to_string())?;
    let bytes = response.bytes().map_err(|e| e.to_string())?;
    std::fs::write(path, bytes).map_err(|e| e.to_string())
}

fn main() {
    tauri::Builder::default()
        .manage(BrowserState {
            history: Mutex::new(Vec::new()),
            bookmarks: Mutex::new(Vec::new()),
        })
        .invoke_handler(tauri::generate_handler![
            navigate,
            record_history,
            get_history,
            add_bookmark,
            get_bookmarks,
            download_file
        ])
        .setup(|app| {
            println!("Zeon Browser started");

            let window = app.get_webview_window("main").unwrap();

            window.eval(
                r#"
                console.log("Rust bridge ready");
                "#,
            )?;

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error running Zeon Browser");
}