/*
 * @Author: qyzzs qyzzzs@163.com
 * @Date: 2023-01-06 11:29:27
 * @LastEditors: qyzzs qyzzzs@163.com
 * @LastEditTime: 2023-01-09 11:16:38
 * @FilePath: \tauri-app\src\router\page\index.ts
 * @Description:
 */
import Layout from "/@/layout/index.vue";

export default [
  {
    path: "/",
    component: Layout,
    redirect: "/index",
    children: [
      {
        path: "index",
        name: "首页",
        component: () => import("/@/views/dashboard/analysis/index.vue"),
      },
    ],
  },
  // {
  //   path: "/:W+",
  //   component: Layout,
  //   redirect: "/404",
  //   hidden: true,
  //   children: [
  //     {
  //       path: "404",
  //       name: "首页",
  //       component: () => import("/@/views/system/error/index.vue"),
  //     },
  //   ],
  // },
];
