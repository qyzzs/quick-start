<!--
 * @Author: qyzzs qyzzzs@163.com
 * @Date: 2023-01-06 11:13:40
 * @LastEditors: qyzzs qyzzzs@163.com
 * @LastEditTime: 2023-01-12 16:30:45
 * @FilePath: \tauri-app\src\layout\sidebar\index.vue
 * @Description: 
-->
<template>
  <a-menu
    id="dddddd"
    v-model:openKeys="openKeys"
    v-model:selectedKeys="selectedKeys"
    mode="inline"
    @click="handleClick"
  >
    <a-sub-menu v-for="menu1 in menuOptions" :key="menu1.name">
      <template #icon>
        <SettingOutlined />
      </template>
      <template #title>{{ menu1.meta.title }}</template>
      <a-menu-item v-for="menu2 in menu1.children" :key="menu2.name">{{ menu2.meta.title }}</a-menu-item>
    </a-sub-menu>
  </a-menu>
</template>
<script lang="ts" setup name="sidebar">
import { ref, watch } from "vue";
import { MailOutlined, QqOutlined, AppstoreOutlined, SettingOutlined } from "@ant-design/icons-vue";
import type { MenuProps } from "ant-design-vue";
import { menuOptions } from "/@/layout/menu";
import router from "/@/router";

const selectedKeys = ref<string[]>([]);
const openKeys = ref<string[]>(['dashboard']);

const handleClick: MenuProps["onClick"] = (e) => {
  router.push(e.key)
};
</script>
