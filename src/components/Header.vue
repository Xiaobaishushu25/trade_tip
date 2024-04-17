<script setup lang="ts">
import {WebviewWindow} from "@tauri-apps/api/webviewWindow";
import {watch, ref} from "vue";
import {invoke} from "@tauri-apps/api/core";
// import axios from "axios";
import {store} from "../store.ts"
import SvgIcon from "./SvgIcon.vue";
import Search from "./Search.vue";
// const max_state_name = ref('window-maximize')
const max_state_name = ref('maximize')
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

async function window_minimize(){
  await WebviewWindow.getCurrent().minimize()
  // console.log(await appWindow.isFocused()) //true
  // setTimeout(async ()=>{
  //   await appWindow.requestUserAttention(UserAttentionType.Critical)
  //   console.log("请求注意")
  // },5000)
}
function window_maximize(){
  max_state.value =!max_state.value
}
async function window_close(){
  await WebviewWindow.getCurrent().hide()
  // await appWindow.close()
}
const keyWord = ref("")
function querySearchAsync(key: string, cb: any){
  // console.log(key)
  // console.log(cb)
  // axios.get(`https://searchadapter.eastmoney.com/api/suggest/get?input=${key}&type=8&token=D43BF722C8E33BDC906FB84D85E326E8&markettype=&mktnum=&jys=&classify=&securitytype=&status=&count=4&_=1712919708063`).then((res)=>{
  invoke("get_response",{url:`https://search-codetable.eastmoney.com/codetable/search/web?clientVersion=lastest&keyword=${key}`}).then((res)=>{
    console.log(res)
    cb(JSON.parse(res).result)
  })
  // axios.get(`https://search-codetable.eastmoney.com/codetable/search/web?clientVersion=lastest&keyword=${key}`).then((res)=>{
  //   console.log(res.data.result)
  //   cb(res.data.result)
  // })
}
const handleSelect = (item: any) => {
  console.log(item)
}
function judgeHaveStock(code: string){
  //遍历stockinfo数组，如果code相等，则返回false ,不渲染
  for(let i = 0; i < store.stockInfo.length; i++){
    if(store.stockInfo[i].code === code){
      return false
    }
  }
  return true
}
function addStock(code: string, name: string){
  invoke("add_stock_info",{code:code,name:name}).then((res)=>{
    console.log(res)
  }).catch((err)=>{
    console.log(err)
  })
}
</script>

<template>
  <div  data-tauri-drag-region class="titlebar"  >
    <Search></Search>
    <div id="stage-button">
      {{max_state_name}}
      <inline-svg src="./src/assets/svg/minimize.svg" class="window-button min" @click.left="window_minimize"></inline-svg>
      <inline-svg :src="`./src/assets/svg/${max_state_name}.svg`" :class="`window-button ${max_state_name}`" @click.left="window_maximize" ></inline-svg>
<!--      <inline-svg src="./src/assets/svg/max.svg" class="window-button max"></inline-svg>-->
      <!--      <inline-svg src="./src/assets/svg/restore.svg" class="window-button restore"></inline-svg>-->
      <inline-svg src="./src/assets/svg/close.svg" class="window-button close"  @click.left="window_close"></inline-svg>

      <!--      <unicon class ="top" viewBox="0 0 1024 1024" name="window-on-top" />-->
<!--      <unicon class ="min" viewBox="0 0 1024 1024" name="window-minimize" @click.left="window_minimize" />-->
<!--      <unicon class ="min" viewBox="0 0 1024 1024" name="window-minimize" @click.left="window_minimize" />-->
<!--      <unicon class ="max" viewBox="0 0 1024 1024" @click.left="window_maximize" :name=max_state_name />-->
<!--      <unicon class ="close" name="multiply" @click.left="window_close" icon-style="monochrome" />-->
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
/*.min path{
  stroke: red;
  stroke-width: 0.5;
}*/
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