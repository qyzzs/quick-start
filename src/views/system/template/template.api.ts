/*
 * @Author: qyzzs qyzzzs@163.com
 * @Date: 2023-01-12 16:26:00
 * @LastEditors: qyzzs qyzzzs@163.com
 * @LastEditTime: 2023-01-17 16:33:31
 * @FilePath: \tauri-app\src\views\system\template\template.api.ts
 * @Description:
 */
import { invoke } from "@tauri-apps/api/tauri";
import { BaseTemplate } from "./template.data";

export function listAll(): Promise<BaseTemplate[]> {
  return invoke<BaseTemplate[]>("list_all");
}

export function addTemplate(params: any) {
  return invoke("add_template", { params: JSON.stringify(params) });
}

export function addConfig(params: any) {
  return invoke("add_config", { params: JSON.stringify(params) });
}

export function addConfigItem(params: any) {
  return invoke("add_config_item", { params: JSON.stringify(params) });
}
