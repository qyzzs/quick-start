/*
 * @Author: qyzzs qyzzzs@163.com
 * @Date: 2023-01-12 17:04:19
 * @LastEditors: qyzzs qyzzzs@163.com
 * @LastEditTime: 2023-01-17 16:47:37
 * @FilePath: \tauri-app\src-tauri\src\base\template_manage\config_options.rs
 * @Description:
 */
use rusqlite::{ Connection };
use serde::{ Deserialize, Serialize };
use super::constant;

#[derive(Debug, Deserialize, Serialize)]
// 选项配置
pub struct BaseTemplateConfigOption {
    pub id: String,
    pub template_config_id: String,
    pub label: String,
    pub value: String,
    pub description: String,
}

pub fn get_config_options(template_config_id: String) -> Option<Vec<BaseTemplateConfigOption>> {
    let conn = Connection::open(constant::DB_NAME).unwrap();
    let mut stmt = conn.prepare(
        format!("SELECT * FROM base_template_config_option WHERE template_config_id = {}", template_config_id).as_str()
    );

    match stmt {
        Ok(mut stmt) => {
            let confit_iter = stmt
                .query_map([], |row| {
                    Ok(BaseTemplateConfigOption {
                        id: row.get(0)?,
                        template_config_id: row.get(1)?,
                        label: row.get(2)?,
                        value: row.get(4)?,
                        description: row.get(5)?,
                    })
                })
                .unwrap();
            let mut result = Vec::new();
            for item in confit_iter {
                match item {
                    Ok(i) => result.push(i),
                    Err(e) => panic!("{}", &e),
                }
            }
            Some(result)
        }
        Err(_e) => None,
    }
}

#[tauri::command]
pub fn add_config_option(params: &str) -> () {
    println!("1111{}", params);
    let template: BaseTemplateConfigOption = serde_json::from_str(params).unwrap();
    let conn = Connection::open(constant::DB_NAME).unwrap();
    let res = conn.execute(
        "
        create table if not exists base_template_config_option (
            id TEXT primary key,
            template_config_id text not null,
            label text not null,
            value text not null,
            description text
        )
    ",
        []
    );
    println!("{:?}", res);

    let result = conn.execute(
        "INSERT INTO base_template_config_option (id, template_config_id, name, key, value, description ) VALUES(?1, ?2, ?3, ?4, ?5, ?6)",
        (
            &template.id,
            &template.template_config_id,
            &template.label,
            &template.value,
            &template.description,
        )
    );
    print!("{:?}", result);
}