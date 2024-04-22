<script setup lang="ts">
import { ref} from "vue";
import {invoke} from "@tauri-apps/api/core";
import StockGroupSelect from "./group/StockGroupSelect.vue";
import StockGroupMange from "./group/StockGroupMange.vue";

const keyWord = ref("")
const nowStock = ref<{ code: string, name: string}>({code:"", name:""})
const dialogFormVisible = ref(false)
const showGroupMange = ref(false)
function querySearchAsync(key: string, cb: any){
  if(key.length === 0){
    cb([])
    return
  }
  // axios.get(`https://searchadapter.eastmoney.com/api/suggest/get?input=${key}&type=8&token=D43BF722C8E33BDC906FB84D85E326E8&markettype=&mktnum=&jys=&classify=&securitytype=&status=&count=4&_=1712919708063`).then((res)=>{
  invoke<string>("get_response",{url:`https://search-codetable.eastmoney.com/codetable/search/web?clientVersion=lastest&keyword=${key}`}).then((res)=>{
    // console.log(res)
    cb(JSON.parse(res).result)
  })
}
const handleSelect = (item: any) => {
  console.log(item)
}

function manageGroup(code: string, name: string){
  showGroupMange.value = !showGroupMange.value
  nowStock.value = {code:code,name:name}
}

const hideDialog = (ok: boolean) => {
  dialogFormVisible.value = false
  if (ok){
    keyWord.value=""
  }
}
</script>

<template>
  <el-autocomplete class="search-input"
                   v-model="keyWord"
                   :clearable = "true"
                   :fetch-suggestions="querySearchAsync"
                   placeholder="输入股票代码、名称、简拼或关键字"
                   @select="handleSelect"
                   value-key="shortName"
  >
    <template #suffix>
      <!--        <inline-svg src="./src/assets/svg/A.svg"></inline-svg>-->
      <inline-svg src="./src/assets/svg/search.svg" class="min-icon" @click="console.log(1)"></inline-svg>
      <!--        <inline-svg :src="require('../assets/A.svg')"></inline-svg>-->
    </template>
    <template #default="{ item }">
      <div class="row">
<!--        <el-tag type="primary">{{ item.securityTypeName }}</el-tag>-->
        <div class="tag">{{item.securityTypeName }}</div>
        <label>{{ `${item.shortName}(${item.code})` }}</label>
        <inline-svg
            src="./src/assets/svg/add.svg"
            class="min-icon add"
            @click.left="manageGroup(item.code,item.shortName)"
        ></inline-svg>
      </div>
    </template>
  </el-autocomplete>
  <StockGroupMange :name="nowStock.name" :code="nowStock.code" :show-dialog="showGroupMange" @hide-dialog="hideDialog"></StockGroupMange>
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