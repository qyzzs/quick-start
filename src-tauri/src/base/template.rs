/*
 * @Author: qyzzs qyzzzs@163.com
 * @Date: 2023-01-09 15:42:17
 * @LastEditors: qyzzs qyzzzs@163.com
 * @LastEditTime: 2023-01-10 22:50:46
 * @FilePath: \tauri-app\src-tauri\src\base\template.rs
 * @Description:
 */
use rusqlite::{ Connection };
use serde::{ Deserialize, Serialize };

#[derive(Debug, Deserialize, Serialize)]
struct BaseTemplate {
    id: String,
    name: String,
    template_config_id: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct BaseTemplateConfig {
    id: String,
    command: String,
    config_item_id: String,
    parameter: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct BaseTemplateConfigItem {
    id: String,
    template_config_id: String,
    name: String,
    key: String,
    value: String,
    require: String,
}

// #[tauri::command]
// pub fn list_all() -> () {
//     let conn = Connection::open("test.db").unwrap();
//     conn.execute(
//         "
//         create table if not exists base_template (
//             id TEXT primary key,
//             name text not null,
//             template_config_id text ,
//         )
//     ",
//         []
//     );

//     let mut _stmt = conn.prepare("SELECT id, name, template_config_id FROM person").unwrap();
//     // let person_iter = stmt.query_map([], |row| {
//     //     Ok(BaseTemplate {
//     //         id: row.get(0)?,
//     //         name: row.get(1)?,
//     //         template_config_id: row.get(2)?,
//     //     })
//     // });

//     // for person in person_iter.iter() {
//     //     println!("Found person {:?}", person.);
//     // }
// }

#[tauri::command]
pub fn add_template(params: &str) -> () {
    println!("1111{}", params);
    let template: BaseTemplate = serde_json::from_str(params).unwrap();
    let conn = Connection::open("test.db").unwrap();
    let res = conn.execute(
        "
        create table if not exists base_template (
            id TEXT primary key,
            name text not null,
            template_config_id text 
        )
    ",
        []
    );
    println!("{:?}", res);

    let result = conn.execute(
        "INSERT INTO base_template (id, name, template_config_id) VALUES(?1, ?2, ?3)",
        (&template.id, &template.name, &template.template_config_id)
    );
    print!("{:?}", result);
}