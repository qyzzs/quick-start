/*
 * @Author: qyzzs qyzzzs@163.com
 * @Date: 2023-01-13 09:09:48
 * @LastEditors: qyzzs qyzzzs@163.com
 * @LastEditTime: 2023-01-17 17:21:35
 * @FilePath: \tauri-app\src\views\system\template\type.ts
 * @Description:
 */


export declare type ConfigOption = BaseTemplateConfig | {};
export declare type Template = BaseTemplate | {};

export interface BaseTemplate {
  id: string;
  name: string;
  description: string;
  config: BaseTemplateConfig;
}

export interface BaseTemplateConfig {
  id: string;
  template_id: string;
  command: string;
  description: string;
  config_options: BaseTemplateConfigOption[];
  config_arguments: BaseTemplateConfigArgument[];
}

export interface BaseTemplateConfigOption {
  id: string;
  template_config_id: string;
  label: string;
  value: string;
  description: string;
}

export interface BaseTemplateConfigArgument {
  id: string;
  template_config_id: string;
  label: string;
  value: string;
  description: string;
}
