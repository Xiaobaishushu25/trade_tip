<script setup lang="ts">
import {onMounted,ref} from "vue";
import {invoke} from "@tauri-apps/api/core";
import {store} from "../store.ts";
import {StockGroup} from "../type.ts";
import StockTable from "./StockTable.vue";
import {VueDraggable} from "vue-draggable-plus";
import InlineSvg from "vue-inline-svg";
const activeName = ref('')
onMounted(() => {
  invoke<StockGroup[]>("query_all_groups",{}).then(res => {
    console.log(res)
    store.stockGroups = res
    activeName.value = store.stockGroups[0].name
    console.log(res)
  }).catch(err => {
    console.log(err)
  })
})
function judgeTab(activeName:string){
  return activeName != '设置';
}
// function handleClick(tab, event:MouseEvent){
//   console.log(tab, event);
// }
</script>

<template>
<!--  <el-tabs v-model="activeName" class="demo-tabs" tab-position="bottom" @tab-click="handleClick" >-->
  <el-tabs v-model="activeName"  tab-position="bottom" :before-leave="judgeTab" >
    <el-tab-pane
        v-for="(group, index) in store.stockGroups"
        :key="index"
        :label="`${group.name}`"
        :name="`${group.name}`"
    >
      <StockTable :stocks_change="group.stocks_change" :group-name="activeName"></StockTable>
    </el-tab-pane>
    <el-tab-pane name="设置">
      <template #label>
        <inline-svg src="./src/assets/svg/menu.svg" class="min-icon" @click="console.log(1)"></inline-svg>
      </template>
    </el-tab-pane>
  </el-tabs>
</template>

<style>
.min-icon:hover path{
  color: green;
  fill: green;
  stroke: green;
}
</style>