<!--
 * @Author: qyzzs qyzzzs@163.com
 * @Date: 2023-01-04 16:01:47
 * @LastEditors: qyzzs qyzzzs@163.com
 * @LastEditTime: 2023-01-10 22:47:05
 * @FilePath: \tauri-app\src\views\dashboard\analysis\index.vue
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

const greetMsg = ref("");
const value = ref('{"id":"123","name":"小明","template_config_id":"111"}');
// const unlisten = await listen("click", (event) => {
//   console.log("event", event);
// });

// emit("click", {
//   theMessage: "ssss",
// });


async function greet() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  greetMsg.value = await invoke("add_template", { params: value.value });
}
</script>
