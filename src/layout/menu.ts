/*
 * @Author: qyzzs qyzzzs@163.com
 * @Date: 2023-01-06 11:17:41
 * @LastEditors: qyzzs qyzzzs@163.com
 * @LastEditTime: 2023-01-09 11:28:10
 * @FilePath: \tauri-app\src\layout\menu.ts
 * @Description:
 */
import type { Router } from "vue-router";
import Layout from "/@/layout/index.vue";
export const menuOptions = [
  {
    name: "dashboard",
    path: "/dashboard",
    component: "/@/layout/index.vue",
    redirect: "/dashboard/analysis",
    meta: {
      componentName: "index",
      keepAlive: false,
      title: "仪表盘",
    },
    children: [
      {
        name: "analysis",
        path: "/analysis",
        meta: {
          componentName: "index",
          keepAlive: false,
          title: "analysis",
        },
        component: "/views/dashboard/analysis/index.vue",
      },
    ],
  },
  {
    name: "system",
    path: "/system",
    component: "/@/layout/index.vue",
    redirect: "/system/template",
    meta: {
      componentName: "index",
      keepAlive: false,
      title: "系统管理",
      icon:'setting-outlined'
    },
    children: [
      {
        name: "template",
        path: "/template",
        meta: {
          componentName: "index",
          keepAlive: false,
          title: "模板管理",
        },
        component: "/views/system/template/index.vue",
      },
    ],
  },
];

export function addRoutes(router: Router) {
  const dynamicViewsModules = import.meta.glob("../views/**/*.{vue,tsx}");
  console.log("router1", router);
  menuOptions
    .map((obj) => ({
      name: obj.name,
      path: obj.path,
      component: Layout,
      redirect: obj.redirect,
      meta: obj.meta,
      children: obj.children.map((c) => ({
        path: c.path,
        name: c.name,
        meta: c.meta,
        component: dynamicImport(dynamicViewsModules, c.component),
      })),
    }))
    .forEach((menu) => {
      router.addRoute(menu);
    });
  console.log("router2", router);
}

function dynamicImport(dynamicViewsModules: any, component: string) {
  const keys = Object.keys(dynamicViewsModules);
  console.log("keys", keys);
  // TODO 可以做的更精确
  const matchKeys = keys.filter((key) => key.includes(component));

  if (matchKeys?.length === 1) {
    return dynamicViewsModules[matchKeys[0]];
  } else if (matchKeys?.length > 1) {
    console.log("warn: 菜单配置错误");
  }
}
