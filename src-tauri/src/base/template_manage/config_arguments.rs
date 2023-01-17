/*
 * @Author: qyzzs qyzzzs@163.com
 * @Date: 2023-01-12 17:04:19
 * @LastEditors: qyzzs qyzzzs@163.com
 * @LastEditTime: 2023-01-17 16:47:41
 * @FilePath: \tauri-app\src-tauri\src\base\template_manage\config_arguments.rs
 * @Description:
 */
use rusqlite::{ Connection };
use serde::{ Deserialize, Serialize };
use super::constant;

#[derive(Debug, Deserialize, Serialize)]
// 参数配置
pub struct BaseTemplateConfigArgument {
    pub id: String,
    pub template_config_id: String,
    pub label: String,
    pub value: String,
    pub description: String,
}

pub fn get_config_arguments(template_config_id: String) -> Option<Vec<BaseTemplateConfigArgument>> {
    println!("11111 get config item");
    let conn = Connection::open(constant::DB_NAME).unwrap();
    let mut stmt = conn.prepare(
        format!("SELECT * FROM base_template_config_argument WHERE template_config_id = {}", template_config_id).as_str()
    );
    match stmt {
        Ok(mut stmt) => {
            let confit_iter = stmt
                .query_map([], |row| {
                    Ok(BaseTemplateConfigArgument {
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
            println!("config_item{:?}", result);
            Some(result)
        }
        Err(_e) => None,
    }
}

#[tauri::command]
pub fn add_config_argument(params: &str) -> () {
    println!("1111{}", params);
    let template: BaseTemplateConfigArgument = serde_json::from_str(params).unwrap();
    let conn = Connection::open(constant::DB_NAME).unwrap();
    let res = conn.execute(
        "
        create table if not exists base_template_config_argument (
            id TEXT primary key,
            template_config_id text not null,
            name text ,
            key text ,
            value text ,
            description text
        )
    ",
        []
    );
    println!("{:?}", res);

    let result = conn.execute(
        "INSERT INTO base_template_config_argument (id, template_config_id, name, key, value, description ) VALUES(?1, ?2, ?3, ?4, ?5)",
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