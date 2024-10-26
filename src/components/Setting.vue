<template>
  <div  data-tauri-drag-region class="titlebar">
    <img src="../assets/icon.png" width="25" height="25" alt="Logo Image" style="margin-left: 5px;margin-right: 10px;user-select: none">
    <label style="font-family: 'Segoe UI'">设置</label>
    <div id="stage-button">
      <inline-svg src="../assets/svg/minimize.svg" class="window-button min" @click.left="window_minimize"></inline-svg>
      <inline-svg src="../assets/svg/close.svg" class="window-button close"  @click.left="window_close"></inline-svg>
    </div>
  </div>
  <el-tabs type="border-card" class="setting-tabs" tabPosition="left" >
    <el-tab-pane label="外观和显示"><DisPlay></DisPlay></el-tab-pane>
    <el-tab-pane label="功能"><FUn></FUn></el-tab-pane>
    <el-tab-pane label="Task">Task</el-tab-pane>
    <el-tab-pane label="Task">Task</el-tab-pane>
    <el-tab-pane label="Task">Task</el-tab-pane>
    <el-tab-pane label="Task">Task</el-tab-pane>
    <el-tab-pane label="Task">Task</el-tab-pane>
    <el-tab-pane label="Task">Task</el-tab-pane>
    <el-tab-pane label="Task">Task</el-tab-pane>
    <el-tab-pane label="Task">Task</el-tab-pane>
  </el-tabs>
</template>

<script lang="ts" setup>
import DisPlay from "./settingComponents/DisPlay.vue";
import {ref, watch} from "vue";
import {WebviewWindow} from "@tauri-apps/api/webviewWindow";
import {saveWindowState, StateFlags} from "@tauri-apps/plugin-window-state";
import FUn from "./settingComponents/FUn.vue";
async function window_minimize(){
  await WebviewWindow.getCurrent().minimize()
}
async function window_close(){
  await saveWindowState(StateFlags.ALL);
  await WebviewWindow.getCurrent().close();
}
</script>
<style>
.el-tabs__item:not(.is-disabled):hover{
  color: black!important;
}
.el-tabs__nav.is-left {
  margin-top: 15px;
}
.el-tabs--left.el-tabs--border-card .el-tabs__item.is-left {
  /* border: 1px solid transparent; */
  /* margin: -1px 0 -1px -1px; */
  margin: 2px;
}
.el-tabs__nav-scroll {
  padding-left: 2px;
  padding-right: 8px;
}
.el-tabs--left.el-tabs--border-card .el-tabs__header.is-left {
  border: 0;
  background-color: #2f2f2f20;
}
</style>
<style scoped>
.setting-tabs{
  min-height: calc(100vh - 31px);
  max-height: calc(100vh - 31px);
}
.setting-tabs :deep(.el-tabs__item) {
  color: black; /* 所有 tab 的字体颜色为黑色 */
  font-size: 15px;
  font-weight: 545;
  text-align: left;
  display: flex;
  justify-content: left;
  align-items: center;
  font-family: "黑体";
}

.setting-tabs :deep(.el-tabs__item.is-active) {
  background-color: #77757550;
  border-radius: 10px;
  color: black;
}

.setting-tabs :deep(.el-tabs__item:not(.is-active):hover) {
  background-color: #77757520;
  border-radius: 10px;
  color: black;
}
.setting-tabs :deep(.el-tabs__active-bar) {
  display: none; /* 去掉选中 tab 下方的线 */
}

.setting-tabs :deep(.el-tabs__item) {
  border: none; /* 去掉 tab 周围的边框 */
}

.setting-tabs :deep(.el-tabs__content) {
  border: none; /* 去掉内容区域的边框 */
}
.setting-tabs > .el-tabs__content {
  font-size: 20px;
}
/* 覆盖 Element UI 的 tab 过渡效果动画时间 */
.setting-tabs :deep(.el-tabs__item){
  transition: background-color 0.05s ease; /* 0.1s 表示 100ms */
}
</style>