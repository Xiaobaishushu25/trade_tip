<script setup lang="ts">
import {onMounted,ref} from "vue";
import {invoke} from "@tauri-apps/api/core";
import {store} from "../store.ts";
import {StockGroup} from "../type.ts";
import StockTable from "./StockTable.vue";
import {VueDraggable} from "vue-draggable-plus";
import DragTest from "./DragTest.vue";
import GroupTest from "./group/GroupTest.vue";
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
function handleClick(tab, event:MouseEvent){
  console.log(tab, event);
}
// const tabs = ref([
//   {
//     title: 'Tab 1',
//     name: 'first',
//   },
//   {
//     title: 'Tab 2',
//     name: 'second',
//   },
//   {
//     title: 'Tab 3',
//     name: 'third',
//   },
// ])
function onDragEnd(event) {
  // 拖拽结束后更新 tabs 顺序
  tabs.value = tabs.value.sort((a, b) => a.name - b.name);
}
const dragging = ref(false);
</script>

<template>
  <n-scrollbar x-scrollable size="50">
    <n-tabs type="card" animated placement="bottom" size="small" >
      <n-tab-pane
          v-for="(group, index) in store.stockGroups"
          :key="index"
          :label="`${group.name}`"
          :name="`${group.name}`"
      >
        Wonderwall
      </n-tab-pane>
    </n-tabs>
  </n-scrollbar>
  <DragTest></DragTest>
  <el-tabs v-model="activeName" class="demo-tabs" tab-position="bottom" @tab-click="handleClick">
    <el-tab-pane label="Task" name="fourth">Task</el-tab-pane>
    <el-tab-pane
        v-for="(group, index) in store.stockGroups"
        :key="index"
        :label="`${group.name}`"
        :name="`${group.name}`"
    >
<!--      <StockTable :stock_codes="group.stock_codes" :group-name="activeName"></StockTable>-->
      <StockTable :stocks_change="group.stocks_change" :group-name="activeName"></StockTable>
    </el-tab-pane>
  </el-tabs>
  <DragTest></DragTest>
<!--  <VueDraggable v-model="store.stockGroups" animation="150" target=".el-tabs">-->
<!--  <VueDraggable v-model="store.stockGroups" animation="150" target=".el-tabs__item">-->
<!--  <el-tabs v-model="activeName"  style="margin-top: 20px;">-->
<!--&lt;!&ndash;      <ElTable :list="list"></ElTable>&ndash;&gt;-->
<!--      <el-tab-pane-->
<!--          v-for="(group, index) in store.stockGroups"-->
<!--          :key="index"-->
<!--          class="cursor-move"-->
<!--          :label="`${group.name}`"-->
<!--          :name="`${group.name}`"-->
<!--      >-->
<!--        {{group.name}}-->
<!--      </el-tab-pane>-->
<!--  </el-tabs>-->
<!--  </VueDraggable>-->
  <GroupTest></GroupTest>
</template>

<style scoped>

</style>