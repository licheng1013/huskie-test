// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use util::http_util::HttpUtil;

use crate::{model::request::Request, util::thread_util::{ ThreadUtil}};
use tauri::Manager;
mod model;
mod util;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn request_test(request: Request) -> u128 {
    println!("请求地址{:?}",request);
    let v = HttpUtil::get_test(request).expect("错误");
    return  v;
}

#[derive(Clone, serde::Serialize)]
struct Payload {
  message: String,
}

fn main() {
    tauri::Builder::default()
    .setup(|app| {
        app.emit_all("click", Payload { message: "Tauri is awesome!".into() }).unwrap();
        Ok(())
      })
        .invoke_handler(tauri::generate_handler![greet])
        .invoke_handler(tauri::generate_handler![request_test])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
