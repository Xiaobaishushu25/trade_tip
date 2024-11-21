<script setup lang="ts">
import RHeader from "./transactionRecordComponents/RHeader.vue";
import RToolBar from "./transactionRecordComponents/RToolBar.vue";
import RTable from "./transactionRecordComponents/RTable.vue";
import {ref, watch} from "vue";

const barRef = ref(null);
const tableRef = ref(null);
const dialogVisible = ref(false)
async function deleteAllRecords(){
  //todo 添加一个确认弹窗
  tableRef.value?.deleteAllRecords();
  dialogVisible.value = false;
}
async function exportRecords(records){
  tableRef.value?.addRecords(records);
}
</script>

<template>
  <el-dialog
      v-model="dialogVisible"
      title="删除全部记录"
      width="500"
  >
    <span>This is a message</span>
    <template #footer>
      <div class="dialog-footer">
        <el-button @click="dialogVisible = false">Cancel</el-button>
        <el-button type="primary" @click="deleteAllRecords">
          Confirm
        </el-button>
      </div>
    </template>
  </el-dialog>
  <div>
    <RHeader></RHeader>
    <RToolBar ref="barRef" @deleteAllRecords="deleteAllRecords" @exportRecords="exportRecords"></RToolBar>
    <RTable ref="tableRef"></RTable>
  </div>
</template>

<style scoped>

</style>