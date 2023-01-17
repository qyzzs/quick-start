<!--
 * @Author: qyzzs qyzzzs@163.com
 * @Date: 2023-01-13 17:20:04
 * @LastEditors: qyzzs qyzzzs@163.com
 * @LastEditTime: 2023-01-17 16:49:57
 * @FilePath: \tauri-app\src\views\system\template\components\templateModal.vue
 * @Description: 
-->
<template>
  <a-button @click="visible = true">addTemplate</a-button>
  <a-modal v-model:visible="visible" title="Add Template" style="width: 420px" @ok="handleOk">
    <a-form
      ref="templateRef"
      :model="templateData"
      name="basic"
      :label-col="{ span: 6 }"
      :wrapper-col="{ span: 16 }"
      autocomplete="off"
    >
      <a-form-item label="模板名称" name="name" :rules="[{ required: true, message: 'Please input template name!' }]">
        <a-input v-model:value="templateData.name" />
      </a-form-item>

      <a-form-item label="描述" name="description" :rules="[{ required: true, message: 'Please input description!' }]">
        <a-input v-model:value="templateData.description" />
      </a-form-item>
    </a-form>
  </a-modal>
</template>
<script lang="ts" setup>
import { ref } from "vue";
import { addTemplate } from "../template.api";
import { BaseTemplate } from "../template.data";

const visible = ref(false);
const templateData = ref({
  name: "",
  description: "",
});
const templateRef = ref();
function handleOk() {
  const update = () => {
    const params = {
      id: "",
      name: templateData.value.name,
      description: templateData.value.description,
    };
    
    let res = addTemplate(params);
    console.log("res", res);
  };
  templateRef.value.validate().then(() => {
    update();
  });
}
</script>
