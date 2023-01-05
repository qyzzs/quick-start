/*
 * @Author: 李帅帅 745677420@qq.com
 * @Date: 2023-01-04 13:17:20
 * @LastEditors: 李帅帅 745677420@qq.com
 * @LastEditTime: 2023-01-04 17:29:57
 * @FilePath: \tauri-app\src-tauri\src\main.rs
 * @Description: 这是默认设置,请设置`customMade`, 打开koroFileHeader查看配置 进行设置: https://github.com/OBKoro1/koro1FileHeader/wiki/%E9%85%8D%E7%BD%AE
 */
#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
mod menu;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    let context = tauri::generate_context!();
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .system_tray(tauri::SystemTray::default())
        .menu(menu::init(&context))
        .on_menu_event(menu::handler)
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
