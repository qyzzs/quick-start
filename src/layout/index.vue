<!--
 * @Author: qyzzs qyzzzs@163.com
 * @Date: 2023-01-06 11:13:40
 * @LastEditors: qyzzs qyzzzs@163.com
 * @LastEditTime: 2023-01-06 18:07:12
 * @FilePath: \tauri-app\src\layout\index.vue
 * @Description: 
-->
<template>
  <div class="app-container">
    <div class="app-header">
      <!-- 顶部导航栏 -->
      <top />
    </div>

    <div class="app-layout">
      <div class="app-left">
        <!-- 左侧导航栏 -->
        <sidebar />
      </div>
      <div class="app-main">
        <!-- 顶部标签卡 -->
        <!-- <tags /> -->
        <!-- 主体视图层     overflow-y:auto;overflow-x:hidden;-->
        <div style="height: 100%" id="app-view">
          <!-- <keep-alive>
            <router-view class="app-view" v-if="$route.meta.keepAlive" />
          </keep-alive> -->
          <router-view class="app-view"/>
        </div>
      </div>
    </div>
  </div>
</template>
<script lang="ts" setup>
import { ref, watch } from "vue";
import { MailOutlined, QqOutlined, AppstoreOutlined, SettingOutlined } from "@ant-design/icons-vue";
import type { MenuProps } from "ant-design-vue";
import top from "./top/index.vue";
import sidebar from "./sidebar/index.vue";

</script>

<style lang="less" scoped>
.app-left,
.app-header,
.app-top,
.app-logo,
.app-layout,
.app-main {
  transition: all 0.3s;
}
.app-container {
  width: 100%;
  height: 100%;
  background: #f0f2f5;
  background-size: 100%;
  background-repeat: no-repeat;
}

.app-header {
  padding-left: 240px;
  width: 100%;
  background-color: #fff;
  box-sizing: border-box;
}
.app-left {
  position: fixed;
  left: 0;
  top: 0;
  width: 240px;
  height: 100%;
  z-index: 1025;
}
.app-main {
    position: absolute;
    left: 240px;
    padding:12px;
    // padding-bottom: 20px;
    width: calc(100% - 240px);
    height: calc(100% - 70px);
    box-sizing: border-box;
    overflow: hidden;
}
.app-view {
  // padding-bottom: 22px;
  width: 100%;
  height: 100%;
  box-sizing: border-box;
}
</style>
