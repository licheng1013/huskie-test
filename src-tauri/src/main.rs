// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use crate::model::request::Request;

mod model;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

// #[tauri::command]
// fn request_test(request: &Request)  {
//     println!("请求地址{:?}",request.request_addr)
// }


fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        //.invoke_handler(tauri::generate_handler![request_test])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
