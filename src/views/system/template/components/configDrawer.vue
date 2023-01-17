<!--
 * @Author: qyzzs qyzzzs@163.com
 * @Date: 2023-01-13 17:44:00
 * @LastEditors: qyzzs qyzzzs@163.com
 * @LastEditTime: 2023-01-17 17:38:38
 * @FilePath: \tauri-app\src\views\system\template\components\configDrawer.vue
 * @Description: 
-->
<template>
  <a-drawer
    class="custom-class"
    style="color: red"
    v-model:visible="visible2"
    :title="config?.name"
    @close="handleClose"
    placement="right"
  >
    <a-form>
      <a-form-item label="命令">
        <a-input style="width: 120px" v-model:value="currentConfig.command"></a-input>
      </a-form-item>
      <a-form-item v-for="item in currentConfig?.config_options" :key="item.id" :label="item.label">
        <div style="display: flex; border-width: 2px">
          <a-input v-model:value="item.label" style="width: 120px" disabled></a-input>-
          <a-input v-model:value="item.value" style="width: 120px"></a-input>
        </div>
      </a-form-item>
      <a-form-item v-bind="formItemLayoutWithOutLabel">
        <a-button type="dashed" style="width: 60%" @click="addDomain">
          <PlusOutlined />
          Add field
        </a-button>
      </a-form-item>
    </a-form>
  </a-drawer>
</template>
<script lang="ts" setup>
import { defineProps, defineEmits, ref, watch } from "vue";
import { Template, BaseTemplateConfig, ConfigOption } from "../type";
import type { PropType } from "vue";
import { PlusOutlined } from "@ant-design/icons-vue";

const emits = defineEmits(["close"]);
const props = defineProps({
  visible: { type: Boolean },
  config: { type: Object as PropType<any> },
});
const formItemLayoutWithOutLabel = ref("");
const visible2 = ref(false);
const currentConfig = ref<BaseTemplateConfig>({
  id: "",
  template_id: "",
  command: "",
  description: "",
  config_options: [],
  config_arguments: [],
});

watch(
  () => props.visible,
  (newVal, oldVal) => {
    if (newVal !== oldVal) visible2.value = newVal;
  },
  { deep: true }
);
watch(
  () => props.config,
  (newVal, oldVal) => {
    if (newVal !== oldVal) {
      currentConfig.value = newVal;
    }
  },
  { deep: true }
);
function addDomain() {}
function handleClose() {
  emits("close");
}
</script>
