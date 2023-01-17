/*
 * @Author: qyzzs qyzzzs@163.com
 * @Date: 2023-01-06 09:14:24
 * @LastEditors: qyzzs qyzzzs@163.com
 * @LastEditTime: 2023-01-10 22:25:47
 * @FilePath: \tauri-app\src-tauri\src\base\base_command.rs
 * @Description:
 */
use serde::{ Deserialize, Serialize };
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

#[derive(Clone, Debug, Serialize, Deserialize)]
struct Options {
    label: String,
    key: String,
    value: String,
    require: String,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct JarConfig {
    cmd: String,
    options: Vec<Options>,
    parameter: String,
}

#[tauri::command]
pub fn get_config(_name: &str) -> JarConfig {
    let path = get_path("jar_config.json");
    let mut file = File::open(path).unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();
    let json: JarConfig = serde_json::from_str(&data).unwrap();
    json
}

fn get_path(file_name: &str) -> PathBuf {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("src");
    path.push("config");
    path.push(file_name);
    path
}