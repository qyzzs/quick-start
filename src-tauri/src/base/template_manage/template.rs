/*
 * @Author: qyzzs qyzzzs@163.com
 * @Date: 2023-01-12 17:04:04
 * @LastEditors: qyzzs qyzzzs@163.com
 * @LastEditTime: 2023-01-17 16:38:50
 * @FilePath: \tauri-app\src-tauri\src\base\template_manage\template.rs
 * @Description:
 */
use rusqlite::{ Connection };
use serde::{ Deserialize, Serialize };
use uuid::Uuid;

use super::config::{ BaseTemplateConfigVO, get_config };
use super::constant;

#[derive(Debug, Deserialize, Serialize)]
pub struct BaseTemplate {
    pub id: String,
    pub name: String,
    pub description: String,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct BaseTemplateVO {
    pub id: String,
    pub name: String,
    pub description: String,
    pub config: Option<BaseTemplateConfigVO>,
}

#[tauri::command]
pub fn list_all() -> Vec<BaseTemplateVO> {
    let conn = Connection::open(constant::DB_NAME).unwrap();

    let mut stmt = conn.prepare("SELECT id, name, description FROM base_template").unwrap();

    let person_iter = stmt
        .query_map([], |row| {
            Ok(BaseTemplateVO {
                id: row.get(0)?,
                name: row.get(1)?,
                description: row.get(2)?,
                config: get_config(row.get(2)?),
            })
        })
        .unwrap();
    let mut result = Vec::new();
    for item in person_iter {
        match item {
            Ok(i) => result.push(i),
            Err(e) => panic!("{}", &e),
        }
    }
    println!("{:?}", result);

    result
}

#[tauri::command]
pub fn add_template(params: &str) -> () {
    println!("add_template params: {}", params);
    let template: BaseTemplate = serde_json::from_str(params).unwrap();
    let conn = Connection::open(constant::DB_NAME).unwrap();
    let res = conn.execute(
        "
        create table if not exists base_template (
            id TEXT primary key,
            name text not null,
            description text 
        )
    ",
        []
    );
    println!("{:?}", res);

    let result = conn.execute(
        "INSERT INTO base_template (id, name, description) VALUES(?1, ?2, ?3)",
        (Uuid::new_v4().to_string().as_str(), &template.name, &template.description)
    );
    print!("{:?}", result);
}