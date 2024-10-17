<script setup lang="ts">
import RHeader from "./transactionRecordComponents/RHeader.vue";
import RToolBar from "./transactionRecordComponents/RToolBar.vue";
import RTable from "./transactionRecordComponents/RTable.vue";
import {ref, watch} from "vue";

const barRef = ref(null);
const tableRef = ref(null);
async function clearFilter(){
  console.log("clearFilter")
  tableRef.value?.clearFilter();
}
</script>

<template>
  <div>
    <RHeader></RHeader>
    <RToolBar ref="barRef" @clearFilter="clearFilter"></RToolBar>
    <RTable ref="tableRef"></RTable>
  </div>
</template>

<style scoped>

</style>