<script setup lang="ts">
import {onMounted,ref} from "vue";
import {invoke} from "@tauri-apps/api/core";
import {store} from "../store.ts";
import {StockGroup} from "../type.ts";
import StockTable from "./StockTable.vue";
import GroupSelect from "./group/GroupSelect.vue";
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
</script>

<template>
  <el-tabs v-model="activeName" class="demo-tabs" tab-position="bottom" @tab-click="handleClick">
    <el-tab-pane label="User" name="first">User</el-tab-pane>
    <el-tab-pane label="Config" name="second">Config</el-tab-pane>
    <el-tab-pane label="Role" name="third">Role</el-tab-pane>
    <el-tab-pane label="Task" name="fourth">Task</el-tab-pane>
    <el-tab-pane
        v-for="(group, index) in store.stockGroups"
        :key="index"
        :label="`${group.name}`"
        :name="`${group.name}`"
    >
      <StockTable :stocks="group.name" :group-name="activeName"></StockTable>
    </el-tab-pane>
  </el-tabs>
  <GroupSelect code="512760"></GroupSelect>
</template>

<style scoped>

</style>