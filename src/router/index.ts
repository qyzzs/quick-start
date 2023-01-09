/*
 * @Author: qyzzs qyzzzs@163.com
 * @Date: 2023-01-04 13:18:11
 * @LastEditors: qyzzs qyzzzs@163.com
 * @LastEditTime: 2023-01-09 10:54:28
 * @FilePath: \tauri-app\src\router\index.ts
 * @Description:
 */
import { createRouter, createWebHistory } from "vue-router";
import PageRouter from "./page/";
import {addRoutes} from "/@/layout/menu";
const routerHistory = createWebHistory();

const router = createRouter({
  history: routerHistory,
  routes: [...PageRouter],
});

addRoutes(router)
export default router;
