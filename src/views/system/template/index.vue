<!--
 * @Author: qyzzs qyzzzs@163.com
 * @Date: 2023-01-06 10:32:35
 * @LastEditors: qyzzs qyzzzs@163.com
 * @LastEditTime: 2023-01-17 17:40:26
 * @FilePath: \tauri-app\src\views\system\template\index.vue
 * @Description: 
-->
<template>
  <TemplateAdd />
  <div class="box">
    <div v-for="template in templateList" :key="template.id" class="item" @click="showDrawer(template)">
      <div style="font-size: 20px; font-weight: bold">{{ template.name }}</div>
      <div>{{ template.description }}</div>
    </div>
  </div>
  <ConfigDrawer :visible="drawerVisible" :config="config" @close="drawerVisible = false" />
</template>

<script lang="ts" setup>
import { ref } from "vue";
import { listAll } from "./template.api";
import { BaseTemplate } from "./type";
import TemplateAdd from "./components/templateModal.vue";
import ConfigDrawer from "./components/configDrawer.vue";

const templateList = ref<BaseTemplate[]>([]);
const drawerVisible = ref(false);
const config = ref<BaseTemplate>();

async function getTemplate() {
  templateList.value = await listAll();
}

function showDrawer(val: BaseTemplate) {
  drawerVisible.value = true;
  config.value = val;
}

getTemplate();
</script>

<style lang="less" scoped>
.box {
  height: 100%;
  width: 100%;
  margin: 12px 0;
  display: flex;

  .item {
    width: 200px;
    height: 80px;
    border-radius: 5px;
    box-shadow: 0 4px 8px 0 rgba(0, 0, 0, 0.2), 0 6px 20px 0 rgba(0, 0, 0, 0.19);
    margin: 0 12px;
    cursor: pointer;
    padding: 12px;
  }
}
</style>
