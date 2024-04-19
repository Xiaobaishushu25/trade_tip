<script setup lang="ts">
import {WebviewWindow} from "@tauri-apps/api/webviewWindow";
import {watch, ref} from "vue";
import {invoke} from "@tauri-apps/api/core";
import {store} from "../store.ts"
import GroupSelect from "./group/GroupSelect.vue";

const keyWord = ref("")
const nowStock = ref<{ code: string, name: string}>({code:"", name:""})
const dialogFormVisible = ref(false)
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
function showDialog(code: string, name: string){
  dialogFormVisible.value = true
  nowStock.value = {code:code,name:name}
  // invoke("add_stock_info",{code:code,name:name}).then((res)=>{
  //   console.log(res)
  // }).catch((err)=>{
  //   console.log(err)
  // })
}
</script>

<template>
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
      <!--        <SvgIcon name="A" color="red" size="20"/>-->
      <!--        <inline-svg src="./src/assets/svg/A.svg"></inline-svg>-->
      <inline-svg src="./src/assets/svg/search.svg" class="min-icon"></inline-svg>
      <!--        <inline-svg :src="require('../assets/A.svg')"></inline-svg>-->
      <!--        <unicon class ="search" viewBox="0 0 1024 1024" name="search" />-->
    </template>
    <template #default="{ item }">
      <div class="row">
        <div class="tag">{{ item.securityTypeName }}</div>
<!--        <label class="link">{{ `${item.shortName}(${item.code})` }}</label>-->
        <label>{{ `${item.shortName}(${item.code})` }}</label>
        <inline-svg
            src="./src/assets/svg/add.svg"
            class="min-icon add"
            v-if="judgeHaveStock(item.code)"
            @click.left="showDialog(item.code,item.shortName)"
        ></inline-svg>
        <!--          <unicon v-if="judgeHaveStock(item.code)" class="icon add" viewBox="0 0 1024 1024" name="add" @click.left="addStock(item.code,item.shortName)" />-->
      </div>
    </template>
  </el-autocomplete>
  <el-dialog v-model="dialogFormVisible" :title="`添加${nowStock.name}`">
    <GroupSelect :code="nowStock.code" :name="nowStock.name"></GroupSelect>
    <template #footer>
      <div class="dialog-footer">
        <el-button @click="dialogFormVisible = false">Cancel</el-button>
        <el-button type="primary" @click="dialogFormVisible = false">
          Confirm
        </el-button>
      </div>
    </template>
  </el-dialog>
</template>

<style>
.row{
  align-items: center; /* 垂直居中 */
  text-align: center;
  cursor: default;
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
.add{
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
</style>