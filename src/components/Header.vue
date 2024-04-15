<script setup lang="ts">
import {WebviewWindow} from "@tauri-apps/api/webviewWindow";
import {watch, ref} from "vue";
import {invoke} from "@tauri-apps/api/core";
// import axios from "axios";
import {store} from "../store.ts"
const max_state_name = ref('window-maximize')
const max_state= ref(false)
watch(max_state, async (newValue) => {
  if(newValue){ //当前状态是最大化
    max_state_name.value = 'window-restore'
    await WebviewWindow.getCurrent().maximize()
  }else{
    max_state_name.value = 'window-maximize'
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
    <el-autocomplete class="search-input"
        v-model="keyWord"
        :clearable = "true"
        :fetch-suggestions="querySearchAsync"
        :trigger-on-focus="false"
        placeholder="输入股票代码、名称、简拼或关键字"
        @select="handleSelect"
        value-key="shortName"
    >
      <template #suffix>
        <unicon class ="search" viewBox="0 0 1024 1024" name="search" />
      </template>
      <template #default="{ item }">
        <div class="row">
          <div class="tag">{{ item.securityTypeName }}</div>
          <label class="link">{{ `${item.shortName}(${item.code})` }}</label>
          <unicon v-if="judgeHaveStock(item.code)" class="icon add" viewBox="0 0 1024 1024" name="add" @click.left="addStock(item.code,item.shortName)" />
          </div>
      </template>
    </el-autocomplete>
    <div id="stage-button">
      <!--      <unicon class ="top" viewBox="0 0 1024 1024" name="window-on-top" />-->
      <unicon class ="min" viewBox="0 0 1024 1024" name="window-minimize" @click.left="window_minimize" />
      <unicon class ="max" viewBox="0 0 1024 1024" @click.left="window_maximize" :name=max_state_name />
      <unicon class ="close" name="multiply" @click.left="window_close" icon-style="monochrome" />
    </div>
  </div>
</template>

<style>
.icon{
  height: 32px;
  width: 32px;
}
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
.search-input{
  width: 40%;
  align-items: center; /* 垂直居中 */
}

.search-input .el-input__inner{
  height: 23px;
  /*border-radius: 0;*/
}
.search-input .el-input__wrapper{
  height: 25px;
}
.search{
  height: 25px;
}
.unicon{ /*为了让最小化、最大化、关闭按钮充满header，不然会小一点*/
  height: 30px;
 }
.add{
  height: 25px;
  width: 25px;
  fill: orange;
  cursor: pointer;
  align-self: center;
}
.el-input__suffix-inner{
  align-items: center; /* 垂直居中 */
}
.tag{
  font-size: 11px;
  color: white;
  background-color: dodgerblue;
  height: 20px;
  width: 35px;
  text-align: center; /* 水平居中（如果需要）*/
  line-height: 20px;
  cursor: default;
}
.row{
  display: flex; /* 确保这是flex容器 */
  align-items: center; /* 垂直居中 */
  justify-content: center;
  text-align: center;
  cursor: default;
}
#stage-button{
  display: flex;
  flex-direction:row;
  justify-content: flex-end;
  /*在 Flexbox 中，margin-left: auto; 会将元素推到其容器的末尾，而 margin-right: 0; 在 Flexbox 中不会产生相同的效果。*/
  margin-left: auto;
}
.top{
  width:30px;
  height: 30px;
}
.top path{
  stroke:red;
  stroke-width:30;
}
.top:hover{
  cursor: pointer;
}
.top:hover path{
  stroke: #37ff00;
  stroke-width:30;
}
.min, .max, .close{
  /*font-size: 30px;用这个无法设置大小*/
  width: 40px;
  height: 30px;
}

.min path{
  stroke: black;
  width: 24px;
  stroke-width: 1;
}
.max path{
  transform: scale(0.7);
  transform-origin: center;
  stroke: black;
  stroke-width:20;
}
.min:hover,.max:hover{
  background-color: #33303020;
  border-radius: 5px;
}

.close:hover{
  fill: white;
  background-color: red;
  border-radius: 5px;
}

</style>