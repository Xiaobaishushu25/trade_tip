<script setup lang="ts">
import {WebviewWindow} from "@tauri-apps/api/webviewWindow";
import {store} from "../store.ts";
import {PaintState} from "../type.ts";
import {emit} from "@tauri-apps/api/event";

async function close() {
  sendPaintState();
  await WebviewWindow.getCurrent().hide();
}
function line() {
  emit('paint', {
    state:PaintState.HL
  })
}
function sendPaintState(state:PaintState = PaintState.Null) {
  emit('paint', {state})
}
function hls() {
  emit('paint', {
    state:PaintState.HLS
  })
}
function setNull() {
  emit('paint', {
    state:PaintState.Null
  })
}
</script>


<template>
<div class="main" data-tauri-drag-region id="draggable-main">
  <img src="../assets/svg/square-close.svg" class="icon" @click="close" alt="水平直线" />
  <div class="two-column">
    <el-button class=min-button @click="sendPaintState(0)">指针</el-button>>
    <el-button class=min-button @click="sendPaintState(1)">水平直线</el-button>>
    <el-button class=min-button @click="sendPaintState(PaintState.HLS)">水平线段</el-button>>
    <el-button class=min-button @click="sendPaintState(PaintState.PHL)">标价直线</el-button>>
    <el-button class=min-button @click="sendPaintState(PaintState.PHLS)">标价线段</el-button>>
    <el-button class=min-button @click="sendPaintState(PaintState.LS)">斜线段</el-button>>
    <el-button class=min-button @click="sendPaintState(PaintState.Text)">文本</el-button>>
    <img src="../assets/svg/point.svg" class="icon" @click="setNull" alt="水平直线" />
    <img src="../assets/svg/hline.svg" class="icon" @click="line" alt="水平直线" />
    <img src="../assets/svg/line.svg" class="icon" alt="水平直线" />
    <img src="../assets/svg/hpartline.svg" class="icon" @click="hls" alt="水平直线" />
    <img src="../assets/svg/A.svg" class="icon" alt="水平直线" />
    <img src="../assets/svg/up.svg" class="icon" alt="水平直线" />
    <img src="../assets/svg/down.svg" class="icon" alt="水平直线" />
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
.icon{
  width: 24px;
  height: 24px;
  cursor: pointer;
  user-select: none;
}
.min-button{
  font-size: 10px;
  width: 60px;
}
</style>