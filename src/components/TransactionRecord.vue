<script setup lang="ts">
import RHeader from "./transactionRecordComponents/RHeader.vue";
import RToolBar from "./transactionRecordComponents/RToolBar.vue";
import RTable from "./transactionRecordComponents/RTable.vue";
import {ref, watch} from "vue";

const barRef = ref(null);
const tableRef = ref(null);
async function deleteAllRecords(){
  //todo 添加一个确认弹窗
  tableRef.value?.deleteAllRecords();
}
async function exportRecords(records){
  tableRef.value?.addRecords(records);
}
</script>

<template>
  <div>
    <RHeader></RHeader>
    <RToolBar ref="barRef" @deleteAllRecords="deleteAllRecords" @exportRecords="exportRecords"></RToolBar>
    <RTable ref="tableRef"></RTable>
  </div>
</template>

<style scoped>

</style>