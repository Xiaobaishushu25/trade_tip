<script setup lang="ts">
import {defineProps, onMounted, ref, watch} from "vue";
import {StockGroup, StockInfo} from "../type.ts";
import {invoke} from "@tauri-apps/api/core";

// defineProps<{groupName: string, stocks:StockInfo[]}>({
const props = defineProps({
  groupName: {
    type: String,
    required: true
  },
  // stocks: {
  //   type: Array as PropType<StockInfo[]>,
  //   required: true
  // }
})
const stockInfo = ref<StockInfo[]>([]);
// watch(groupName, (newValue, oldValue) => {
//   console.log(newValue, oldValue)
// })
watch(() => props.groupName, (newValue, oldValue) => {
  console.log('groupName changed:', newValue, oldValue);
  invoke<StockInfo[]>("query_stocks_by_group_name", {name: newValue}).then(res => {
    console.log(res);
    stockInfo.value = res;
    console.log(stockInfo.value)
  }).catch(err => {
    console.log(err);
  })
});
onMounted(() => {
  console.log(props.groupName)
  const groupName = props.groupName;
  invoke<StockInfo[]>("query_stocks_by_group_name", {name: groupName}).then(res => {
    console.log(res);
    stockInfo.value = res;
    console.log(stockInfo.value)
  }).catch(err => {
    console.log(err);
  })
})
const tableRowClassName = ({row, rowIndex,}:{
  row: StockInfo
  rowIndex: number
}) => {
  if (rowIndex === 1) {
    return 'warning-row'
  } else if (rowIndex === 3) {
    return 'success-row'
  }
  return ''
}
</script>

<template>
  <el-table
      :data="stockInfo"
      style="width: 100%"
      :row-class-name="tableRowClassName"
  >
    <el-table-column prop="stock_code" label="代码" width="180" />
    <el-table-column prop="stock_name" label="名称" width="180" />
    <el-table-column prop="price" label="现价" />
    <el-table-column prop="change_percent" label="涨跌%" />
  </el-table>
</template>

<style scoped>

</style>