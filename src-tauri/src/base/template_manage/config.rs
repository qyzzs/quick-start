/*
 * @Author: qyzzs qyzzzs@163.com
 * @Date: 2023-01-12 17:04:11
 * @LastEditors: qyzzs qyzzzs@163.com
 * @LastEditTime: 2023-01-17 16:47:28
 * @FilePath: \tauri-app\src-tauri\src\base\template_manage\config.rs
 * @Description:
 */
use rusqlite::{ Connection };
use serde::{ Deserialize, Serialize };
use super::config_options::{ BaseTemplateConfigOption, get_config_options };
use super::config_arguments::{ BaseTemplateConfigArgument, get_config_arguments };
use super::constant;

#[derive(Debug, Deserialize, Serialize)]
pub struct BaseTemplateConfig {
    pub id: String,
    pub template_id: String,
    pub command: String,
    pub description: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BaseTemplateConfigVO {
    pub id: String,
    pub template_id: String,
    pub command: String,
    pub description: String,
    pub config_options: Option<Vec<BaseTemplateConfigOption>>,
    pub config_arguments: Option<Vec<BaseTemplateConfigArgument>>,
}

pub fn get_config(template_id: String) -> Option<BaseTemplateConfigVO> {
    let conn = Connection::open(constant::DB_NAME).unwrap();
    let mut stmt = conn.prepare(
        format!("SELECT * FROM base_template_config where id = {} limit 1", template_id).as_str()
    );
    match stmt {
        Ok(mut stmt) => {
            let config = stmt
                .query_row([], |row| {
                    Ok(BaseTemplateConfigVO {
                        id: row.get(0)?,
                        template_id: row.get(1)?,
                        command: row.get(2)?,
                        description: row.get(3)?,
                        config_options: get_config_options(row.get(0)?),
                        config_arguments: get_config_arguments(row.get(0)?),
                    })
                })
                .unwrap();
            Some(config)
        }
        Err(_e) => { None }
    }
}

#[tauri::command]
pub fn add_config(params: &str) -> () {
    println!("1111{}", params);
    let config: BaseTemplateConfig = serde_json::from_str(params).unwrap();
    let conn = Connection::open(constant::DB_NAME).unwrap();
    let res = conn.execute(
        "
        create table if not exists base_template_config (
            id TEXT primary key,
            template_id not null ,
            command text not null ,
            description text, 
        )
    ",
        []
    );
    println!("res {:?}", res);

    let result = conn.execute(
        "INSERT INTO base_template_config (id, template_id, command, description) VALUES(?1, ?2, ?3, ?4)",
        (&config.id, &config.template_id, &config.command, &config.description)
    );
    print!("result:{:?}", result);
}