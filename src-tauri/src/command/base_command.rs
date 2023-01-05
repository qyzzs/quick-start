use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Read;
use std::path;
use std::path::Path;
use std::path::PathBuf;

#[derive(Clone, Debug, Serialize, Deserialize)]
struct JarConfig {
    message: String,
}

#[tauri::command]
pub fn get_config(name: &str) -> String {
    let path = get_path(Path::new("jar_config.json"));
    let mut file = File::open(path).unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();
    let json: JarConfig = serde_json::from_str(&data).unwrap();

    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn get_path(config_name: &Path) -> PathBuf {
    let mut config_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    config_path.push("src");
    config_path.push("config");
    config_path.push(config_name);
    config_path
}
