/*
 * @Author: qyzzs qyzzzs@163.com
 * @Date: 2023-01-04 13:17:20
 * @LastEditors: qyzzs qyzzzs@163.com
 * @LastEditTime: 2023-01-05 17:23:44
 * @FilePath: \tauri-app\src-tauri\src\main.rs
 * @Description:
 */

#![cfg_attr(all(not(debug_assertions), target_os = "windows"), windows_subsystem = "windows")]
mod menu;
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    let context = tauri::generate_context!();

    tauri::Builder
        ::default()
        .invoke_handler(tauri::generate_handler![greet])
        .system_tray(menu::system_tray_menu::menu())
        .on_system_tray_event(menu::system_tray_menu::handler)
        .menu(menu::app_menu::init(&context))
        .on_menu_event(menu::app_menu::handler)
        .run(context)
        .expect("error while running tauri application");
}