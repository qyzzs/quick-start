<!--
 * @Author: qyzzs qyzzzs@163.com
 * @Date: 2023-01-04 16:01:47
 * @LastEditors: qyzzs qyzzzs@163.com
 * @LastEditTime: 2023-01-06 13:13:17
 * @FilePath: \tauri-app\src\views\home\index.vue
 * @Description: 
-->

<template>
  <div style="padding: 12px">
    <a-space>
      <a-input v-model:value="value" placeholder="Basic usage" />
      <a-button @click="greet">ddd</a-button>
    </a-space>
    {{ greetMsg }}
  </div>
  <!-- <GreetVue/> -->
</template>

<script setup lang="ts">
import { ref } from "vue";
import {invoke} from '@tauri-apps/api/tauri'
// import GreetVue from "/@/components/Greet.vue";
// import { emit, listen } from "@tauri-apps/api/event";

const value = ref("");
const greetMsg = ref("");
// const unlisten = await listen("click", (event) => {
//   console.log("event", event);
// });

// emit("click", {
//   theMessage: "ssss",
// });

async function greet() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  greetMsg.value = await invoke("get_config", { name: value.value });
}
</script>
