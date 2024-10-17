<script setup lang="ts">
import {TransactionRecord} from "../../type.ts";
import {TableColumnCtx} from "element-plus";
import {ref, watch} from "vue";


const tableRef = ref(null);
let transactionRecords = [
  {
    date: "2024-10-01",
    time: "09:30",
    code: "600000",
    name: "浦发银行",
    direction: "买入",
    num: 100,
    price: 12.34,
    amount: 1234.00,
    remark: "无"
  },
  {
    date: "2024-10-02",
    time: "10:00",
    code: "000001",
    name: "平安银行",
    direction: "卖出",
    num: 50,
    price: 20.56,
    amount: 1028.00,
    remark: "无"
  },
  {
    date: "2024-10-03",
    time: "11:15",
    code: "300750",
    name: "宁德时代",
    direction: "买入",
    num: 30,
    price: 200.00,
    amount: 6000.00,
    remark: "无"
  },
  {
    date: "2024-10-04",
    time: "14:45",
    code: "600519",
    name: "贵州茅台",
    direction: "卖出",
    num: 10,
    price: 1500.00,
    amount: 15000.00,
    remark: "无"
  },
  {
    date: "2024-10-05",
    time: "15:30",
    code: "601318",
    name: "中国平安",
    direction: "买入",
    num: 5,
    price: 70.00,
    amount: 350.00,
    remark: "无"
  },
];
const filteredRecords = ref(transactionRecords);
let selectedCode = '0';
const tableRowClassName = ({row}: {
  row: TransactionRecord,
}) => {
  if (row.direction.includes('买入')){
    return 'red-row'
  }else if (row.direction.includes('卖出')){
    return 'blue-row'
  }
  return 'black-row'
}
// const remarkFormatter = (row: TransactionRecord) => {
//   return row.remark ? row.remark : '无备注';
// }
async function codeFilter(specifiedCode:string) {
  filteredRecords.value = transactionRecords.filter(record => record.code === specifiedCode);
}
async function clearFilter() {
  filteredRecords.value = transactionRecords;

  selectedCode = '0';
}
// async function addRecords(records: TransactionRecord[]) {
//   console.log("表格收到了数据")
//   console.log(records)
//   transactionRecords.push(...records);
//   console.log(transactionRecords)
//   if (selectedCode!== '0'){
//     filteredRecords.value.push(records.filter(record => record.code === selectedCode));
//   }else {
//     filteredRecords.value = transactionRecords;
//   }
//   console.log(filteredRecords.value)
// }
async function addRecords(records: TransactionRecord[]) {
  console.log("表格收到了数据");
  console.log(records);

  // 追加数据到 transactionRecords
  transactionRecords.push(...records);

  console.log(transactionRecords);

  // 判断 selectedCode，如果不是 '0' 则过滤后赋值，否则直接赋值全部数据
  if (selectedCode !== '0') {
    // 这里用新的数组对象来触发 Vue 的响应式(push 不会触发,可能因为没深度监听？)
    filteredRecords.value = transactionRecords.filter(record => record.code === selectedCode);
  } else {
    // 直接重新赋值整个数组来触发响应式
    filteredRecords.value = [...transactionRecords];
  }
  console.log(filteredRecords.value);
}
defineExpose({ codeFilter, clearFilter, addRecords})
</script>

<template>
  <el-table
      ref="tableRef"
      :data="filteredRecords"
      :row-class-name="tableRowClassName"
      style="width: 100%;font-size: 14px"
  >
    <el-table-column prop="date" label="交易日期" sortable style="font-size: 14px" width="110" />
    <el-table-column prop="time" label="交易时间" sortable style="font-size: 14px" width="90" />
    <el-table-column prop="code" label="证券代码" sortable style="font-size: 14px" width="90" />
    <el-table-column prop="name" label="证券名称" sortable style="font-size: 14px" width="90" />
    <el-table-column prop="direction" label="交易类别" sortable style="font-size: 14px" width="90" />
    <el-table-column prop="num" label="交易数量" class-name="right-cell" style="font-size: 14px" width="80" />
    <el-table-column prop="price" label="成交价格" class-name="right-cell" style="font-size: 14px" width="80" />
    <el-table-column prop="amount" label="成交金额" sortable class-name="right-cell" style="font-size: 14px" width="90" />
<!--    <el-table-column prop="remark" label="备注" :formatter="remarkFormatter" style="font-size: 14px" width="160" />-->
    <el-table-column prop="remark" label="备注"  style="font-size: 14px" width="160" />
  </el-table>
  <button @click="codeFilter('600000')">ce</button>
  <button @click="clearFilter">ce</button>
  <label>{{selectedCode}}</label>
  <label>{{transactionRecords}}</label>
</template>

<style>
.el-table th {
  color: black; /* 表头字体为黑色 */
  text-align: center;
}
.el-table .cell{
  padding-left: 5px;
  padding-right: 5px;
}
.el-table .red-row .cell {
  color: #c30909; /* 红色文本 */
}
.el-table .blue-row .cell {
  color: blue; /* 蓝色文本 */
}
.el-table .right-cell {
  text-align: right; /* 或者根据需要设置为其他对齐方式 */
}
</style>