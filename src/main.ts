/*
 * @Author: 李帅帅 745677420@qq.com
 * @Date: 2023-01-04 13:17:20
 * @LastEditors: 李帅帅 745677420@qq.com
 * @LastEditTime: 2023-01-04 16:00:51
 * @FilePath: \tauri-app\src\main.ts
 * @Description: 这是默认设置,请设置`customMade`, 打开koroFileHeader查看配置 进行设置: https://github.com/OBKoro1/koro1FileHeader/wiki/%E9%85%8D%E7%BD%AE
 */
import { createApp } from "vue";
import "./style.css";
import App from "./App.vue";
import router from './router'

createApp(App).use(router).mount("#app");
