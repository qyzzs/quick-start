/*
 * @Author: qyzzs qyzzzs@163.com
 * @Date: 2023-01-05 16:30:30
 * @LastEditors: qyzzs qyzzzs@163.com
 * @LastEditTime: 2023-01-05 18:13:17
 * @FilePath: \tauri-app\src-tauri\src\command\command_builder.rs
 * @Description:
 */
use std::process::Command;

 struct Cmd {
    command: String,
    options: String,
    parameter: String,
}

impl Cmd {
    pub fn new(command: String, options: String, parameter: String) -> String {
        Cmd { command, options, parameter }
    }

    fn init(&self) -> String {
        command + options + parameter
    }

    pub fn exec(&self) -> String {
        let cmd = init();
        Command.new(self.command).arg(self.options);
    }
}