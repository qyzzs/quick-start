/*
 * @Author: qyzzs qyzzzs@163.com
 * @Date: 2023-01-04 13:17:20
 * @LastEditors: qyzzs qyzzzs@163.com
 * @LastEditTime: 2023-01-13 16:26:07
 * @FilePath: \tauri-app\src-tauri\src\main.rs
 * @Description:
 */

#![cfg_attr(all(not(debug_assertions), target_os = "windows"), windows_subsystem = "windows")]
mod base;
mod menu;
use base::template_manage::*;
use base::command;

fn main() {
    let context = tauri::generate_context!();

    tauri::Builder
        ::default()
        .invoke_handler(
            tauri::generate_handler![
                template::add_template,
                config::add_config,
                config_options::add_config_option,
                config_arguments::add_config_argument,
                command::base_command::get_config,
                template::list_all
            ]
        )
        .system_tray(menu::system_tray_menu::menu())
        .on_system_tray_event(menu::system_tray_menu::handler)
        .menu(menu::app_menu::init(&context))
        .on_menu_event(menu::app_menu::handler)
        .run(context)
        .expect("error while running tauri application");
}