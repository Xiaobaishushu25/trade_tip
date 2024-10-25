<script setup lang="ts">
import {WebviewWindow} from "@tauri-apps/api/webviewWindow";
import {PaintState} from "../type.ts";
import {emit} from "@tauri-apps/api/event";
import {ref} from "vue";


const activeButton = ref("button0"); // 用于跟踪哪个按钮被点击
async function close() {
  await sendPaintState();
  await WebviewWindow.getCurrent().hide();
}
async function sendPaintState(state:PaintState = PaintState.Null) {
  await emit('paint', {state});
  activeButton.value = 'button' + state; // 假设你的按钮有对应的CSS类名
}
</script>


<template>
<div class="main" data-tauri-drag-region id="draggable-main">
  <div class="icon-row" data-tauri-drag-region>
    <img src="../assets/svg/square-close.svg" class="icon icon-right" @click="close" alt="水平直线" />
  </div>
  <div class="two-column">
    <el-button class="min-button" :class="{ 'active-button': activeButton === 'button0' }"   @click="sendPaintState(0)">指针指针</el-button>
    <el-button class="min-button" :class="{ 'active-button': activeButton === 'button1' }"  @click="sendPaintState(1)">水平直线</el-button>
    <el-button class="min-button" :class="{ 'active-button': activeButton === 'button2' }"  @click="sendPaintState(PaintState.HLS)">水平线段</el-button>
    <el-button class="min-button" :class="{ 'active-button': activeButton === 'button3' }"  @click="sendPaintState(PaintState.PHL)">标价直线</el-button>
    <el-button class="min-button" :class="{ 'active-button': activeButton === 'button4' }"  @click="sendPaintState(PaintState.PHLS)">标价线段</el-button>
    <el-button class="min-button" :class="{ 'active-button': activeButton === 'button5' }"  @click="sendPaintState(PaintState.LS)">斜线段</el-button>
    <el-button class="min-button" :class="{ 'active-button': activeButton === 'button6' }"  @click="sendPaintState(PaintState.Text)">文本</el-button>
<!--    <img src="../assets/svg/point.svg" class="icon"  alt="水平直线" />-->
<!--    <img src="../assets/svg/hline.svg" class="icon"  alt="水平直线" />-->
<!--    <img src="../assets/svg/line.svg" class="icon" alt="水平直线" />-->
<!--    <img src="../assets/svg/hpartline.svg" class="icon" alt="水平直线" />-->
<!--    <img src="../assets/svg/A.svg" class="icon" alt="水平直线" />-->
<!--    <img src="../assets/svg/up.svg" class="icon" alt="水平直线" />-->
<!--    <img src="../assets/svg/down.svg" class="icon" alt="水平直线" />-->
  </div>
</div>
</template>

<style scoped>
.main{
  width: 100%;
  height: 100vh;
  background-color: #ecece4;
}
.two-column{
  display: grid;
  grid-template-columns: repeat(2, 1fr); /* 创建两列 */
  grid-gap: 2px; /* 列与列之间的间距 */
}
.two-column .min-button:first-child {
  margin-left: 12px; /* 或者你想要的任何值 */
}
.icon{
  width: 24px;
  height: 24px;
  cursor: pointer;
  user-select: none;
}
.icon-row{
  display: flex;
  justify-content: flex-end;
}
.min-button{
  font-size: 10px;
  width: 60px;
}
.active-button{
  background-color: #24c8db;
  color: white;
}
</style>