<script setup lang="ts">
import {WebviewWindow} from "@tauri-apps/api/webviewWindow";
import {saveWindowState, StateFlags} from "@tauri-apps/plugin-window-state";
import PositionChart from "./PositionComponents/PositionChart.vue";
import PositionInsert from "./PositionComponents/PositionUpdate.vue";

async function window_minimize(){
  await WebviewWindow.getCurrent().minimize()
}
async function window_close(){
  await saveWindowState(StateFlags.ALL);
  await WebviewWindow.getCurrent().close();
}
</script>

<template>
  <div  data-tauri-drag-region class="titlebar">
    <img src="../assets/icon.png" width="25" height="25" alt="Logo Image" style="margin-left: 5px;margin-right: 10px;user-select: none">
    <label style="font-family: 'Segoe UI'">仓位变化</label>
    <div id="stage-button">
      <inline-svg src="../assets/svg/minimize.svg" class="window-button min" @click.left="window_minimize"></inline-svg>
      <inline-svg src="../assets/svg/close.svg" class="window-button close"  @click.left="window_close"></inline-svg>
    </div>
  </div>
  <div class="content column">
    <position-chart></position-chart>
    <position-insert></position-insert>
  </div>
</template>

<style scoped>

</style>