<script setup lang="ts">
import {onMounted,ref,watch} from "vue";
import {invoke} from "@tauri-apps/api/core";
import {store} from "../store.ts";
import {StockGroup, StockInfoG, StockLiveData} from "../type.ts";
import StockTable from "./StockTable.vue";
import {VueDraggable} from "vue-draggable-plus";
import InlineSvg from "vue-inline-svg";
import {listen} from "@tauri-apps/api/event";
const activeName = ref('')
onMounted(async () => {
  invoke<StockGroup[]>("query_all_groups",{}).then(res => {
    store.stockGroups = res
    activeName.value = store.stockGroups[0].name
  }).catch(err => {
    console.log(err)
  });
  query_box();
  await listen("graphic_change", () => {
    query_box();
  })
})
function query_box(){
  invoke<Record<string, number[]>>("query_box",{}).then(res => {
    store.boxData = res
  }).catch(err => {
    console.log(err)
  })
}
watch(activeName, () => {
  console.log("开始实时查询")
  let groupName = activeName.value;
  store.activeGroup = groupName;
  invoke("query_live_stocks_data",{groupName: groupName}).catch(err => {
    console.log(err);
  });
  invoke<StockInfoG[]>("query_stocks_by_group_name", {name: groupName}).then(res => {
    store.stockinfoGs = res;
  }).catch(err => {
    console.log(err);
  })
},{immediate:true})

function judgeTab(activeName:string){
  return activeName != '设置';
}
// function handleClick(tab, event:MouseEvent){
//   console.log(tab, event);
// }
</script>

<template>
  <div class="tab-container">
    <!--  <el-tabs v-model="activeName" class="demo-tabs" tab-position="bottom" @tab-click="handleClick" >-->
    <el-tabs v-model="activeName"  tab-position="bottom" :before-leave="judgeTab" >
      <el-tab-pane
          v-for="(group, index) in store.stockGroups"
          :key="index"
          :label="`${group.name}`"
          :name="`${group.name}`"
          class="tab-pane"
      >
<!--        <StockTable :stocks_change="group.stocks_change" :group-name="activeName"></StockTable>-->
        <StockTable :stocks_change="group.stocks_change" :group-name="group.name" :active-name="activeName"></StockTable>
      </el-tab-pane>
      <el-tab-pane name="设置">
        <template #label>
          <inline-svg src="./src/assets/svg/menu.svg" class="min-icon" @click="console.log(1)"></inline-svg>
        </template>
      </el-tab-pane>
    </el-tabs>
  </div>
</template>

<style>
.tab-container{
  height: 100%;
}
.el-tabs__nav-scroll{
  padding-left: 20px;
}
.tab-pane{
  min-height: 200px;
}
.min-icon:hover path{
  color: green;
  fill: green;
  stroke: green;
}
</style>