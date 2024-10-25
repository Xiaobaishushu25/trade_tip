<script setup lang="ts">
import {WebviewWindow} from "@tauri-apps/api/webviewWindow";
import {ref, watch} from "vue";
import {invoke} from "@tauri-apps/api/core";
import {store} from "../store.ts"
import Search from "./Search.vue";
import {useRouter} from "vue-router";
import {saveWindowState, StateFlags} from "@tauri-apps/plugin-window-state";

const router = useRouter()
const max_state_name = ref('maximize')
const live_name = ref('stop')
const live_state = ref(true)
const max_state= ref(false)
watch(max_state, async (newValue) => {
  if(newValue){ //当前状态是最大化
    max_state_name.value = 'restore'
    await WebviewWindow.getCurrent().maximize()
  }else{
    max_state_name.value = 'maximize'
    await WebviewWindow.getCurrent().unmaximize()
  }
})
watch(live_state, async (newValue) => {
  if(newValue){ //当前状态是最大化
    live_name.value = 'stop'
  }else{
    live_name.value = 'start'
  }
})
async function changeUpdateState(){
  await invoke('update_live_state',{groupName:store.activeGroup,liveState:!live_state.value});
  live_state.value = !live_state.value;
}
async function open_record(){
  const webview = new WebviewWindow('record', {
    url: '/#/record',
    center: true,
    title: '交易记录',
    width: 900,
    height: 700,
    minWidth: 800,
    minHeight: 625,
    decorations: false,
    resizable: true,
    dragDropEnabled: false,
    visible: false,
  });
  await webview.show()
}
async function open_setting(){
  const webview = new WebviewWindow('setting', {
    url: '/#/setting',
    center: true,
    title: '设置',
    width: 1025,
    height: 800,
    minWidth: 1025,
    minHeight: 625,
    decorations: false,
    resizable: true,
    dragDropEnabled: false,
    visible: false,
  });
  // await webview.once('tauri://created', function () {
  //   console.log("webview successfully created")
  // });
  // await webview.once('tauri://error', function (e) {
  //   console.log("an error happened creating the webview", e)
  // });
  await webview.show()
  // // const appWindow = await WebviewWindow.getByLabel('setting')
  // console.log(webview)
  // await webview?.show()
}
async function window_minimize(){
  // console.log("窗口是",await WebviewWindow.getCurrent().isResizable());
  await WebviewWindow.getCurrent().minimize()
}
function window_maximize(){
  max_state.value =!max_state.value
}
async function window_close(){
  // await WebviewWindow.getCurrent().hide()
  // const ALL_WITHOUT_VISIBLE = StateFlags.ALL & ~StateFlags.VISIBLE;
  // await saveWindowState(ALL_WITHOUT_VISIBLE);
  console.log("退出程序");
  await saveWindowState(StateFlags.ALL);
  await invoke('exit_app', {})
  await WebviewWindow.getCurrent().close();
}
function back(){
  router.back();
}
</script>

<template>
  <div  data-tauri-drag-region class="titlebar"  >
<!--    <Search></Search>-->
    <img src="../assets/icon.png" width="25" height="25" alt="Logo Image" style="margin-left: 5px;margin-right: 10px;user-select: none">
    <Search></Search>
    <div id="stage-button">
      <el-tooltip content="交易记录" placement="bottom" effect="light" :show-arrow="false">
        <inline-svg src="../assets/svg/record.svg" class="window-button back record" @click.left="open_record"></inline-svg>
      </el-tooltip>
      <el-tooltip content="设置" placement="bottom" effect="light" :show-arrow="false">
        <inline-svg src="../assets/svg/setting.svg" class="window-button back" @click.left="open_setting"></inline-svg>
      </el-tooltip>
      <el-tooltip :content="`${live_name}实时更新`" placement="bottom" effect="light" :show-arrow="false">
        <inline-svg :src="`../assets/svg/${live_name}.svg`" class="window-button back" @click.left="changeUpdateState"></inline-svg>
      </el-tooltip>
      <el-tooltip content="返回" placement="bottom" effect="light" :show-arrow="false">
        <inline-svg src="../assets/svg/back.svg" class="window-button back" @click.left="back()"></inline-svg>
      </el-tooltip>
      <inline-svg src="../assets/svg/minimize.svg" class="window-button min" @click.left="window_minimize"></inline-svg>
      <inline-svg :src="`../assets/svg/${max_state_name}.svg`" :class="`window-button ${max_state_name}`" @click.left="window_maximize" ></inline-svg>
      <inline-svg src="../assets/svg/close.svg" class="window-button close"  @click.left="window_close"></inline-svg>
    </div>
  </div>
</template>

<style>
.titlebar{
  align-items: center; /* 垂直居中 */
  display: flex;
  flex-direction:row;
  height: 30px;
  user-select: none;
  background-color: #ecece4;
  /*top: 0px;
  left: 0;
  right: 0;*/
}
.window-button{/*去掉加上tooltip后出现的黑色边框*/
  outline: none !important;
}
.record path{
  transform: scale(0.9);
  transform-origin: center;
  stroke-width: 20;
}
#stage-button{
  display: flex;
  flex-direction:row;
  justify-content: flex-end;
  /*在 Flexbox 中，margin-left: auto; 会将元素推到其容器的末尾，而 margin-right: 0; 在 Flexbox 中不会产生相同的效果。*/
  margin-left: auto;
}
.window-button{
  height: 30px;
  width: 40px;
}
.min path{
  transform: scale(0.6);
  transform-origin: center;
}
.maximize path{
  transform: scale(0.7);
  transform-origin: center;
}
.restore path{
  transform: scale(0.8);
  transform-origin: center;
  stroke-width: 20;
  stroke: #0f0f0f;
}

.back:hover path{
  fill: #12d912;
}
.min:hover,.maximize:hover,.restore:hover{
  background-color: #33303020;
  border-radius: 5px;
}
.close path{
  transform: scale(0.8);
  transform-origin: center;
  stroke-width: 20;
  stroke: #0f0f0f;
}
.close:hover{
  background-color: red;
  border-radius: 5px;
}
.close:hover path{
  fill: white;
  stroke: white;
}
</style>