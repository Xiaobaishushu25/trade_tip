<script setup lang="ts">
import {TransactionRecord} from "../../type.ts";
import {ref,reactive, watch,onMounted,nextTick,} from "vue";
import {invoke} from "@tauri-apps/api/core";
import {errorNotification} from "../../utils.ts";


const tableRef = ref(null);
const tableRowInputRef: any = ref(null)
onMounted(() => {
  console.log("table mounted")
  invoke<TransactionRecord[]>('query_transaction_records',{}).then(data => {
    console.log(data);
    // transactionRecords = data;
    addRecords(data);
  }).catch(e => {
    console.error(e)
  })
})
let transactionRecords: TransactionRecord[] = [];
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
async function codeFilter(specifiedCode:string) {
  console.log("table codeFilter", specifiedCode);
  selectedCode = specifiedCode;
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
  console.log(records);
  // 追加数据到 transactionRecords
  // transactionRecords.push(...records);
  // 从前面插入数据到 transactionRecords
  transactionRecords.unshift(...records);
  // 判断 selectedCode，如果不是 '0' 则过滤后赋值，否则直接赋值全部数据
  if (selectedCode !== '0') {
    // 这里用新的数组对象来触发 Vue 的响应式(push 不会触发,可能因为没深度监听？)
    filteredRecords.value = transactionRecords.filter(record => record.code === selectedCode);
  } else {
    // 直接重新赋值整个数组来触发响应式
    filteredRecords.value = [...transactionRecords];
  }
}
const state = reactive({
  tableList: [], // 表单数据
  // 编辑的表格行
  tableRowEditIndex: undefined,
  // 编辑的表格列
  tableColumnEditIndex: undefined
})
// 双击可编辑的单元格
const dbClickCell = (scope: any) => {
  // 找到单个格子独有的属性 - 这里是用所在行跟列id区别显示
  state.tableRowEditIndex = scope.$index
  state.tableColumnEditIndex = scope.column.id
  nextTick(() => {
    // 获取焦点
    tableRowInputRef.value.focus()
  })
}
// 表格中input取消焦点,select变化
const onInputTableBlur = async (scope: any) => {
  state.tableRowEditIndex = undefined
  state.tableColumnEditIndex = undefined
  let data = scope.row;
  console.log(data)
  if (data.remark !== '双击编辑') {
    let result = await invoke('update_transaction_record',{record: data});
    if (result!=null) {
      errorNotification(`更新备注失败：${result}`)
    }
  }
}

defineExpose({ codeFilter, clearFilter, addRecords})
</script>

<template>
  <el-table
      ref="tableRef"
      :data="filteredRecords"
      :row-class-name="tableRowClassName"
      max-height="calc(100vh - 90px)"
      style="width: 100%;font-size: 14px;padding-left: 20px"
  >
    <el-table-column prop="date" label="交易日期" sortable style="font-size: 14px" width="100" />
    <el-table-column prop="time" label="交易时间" sortable style="font-size: 14px" width="90" />
    <el-table-column prop="code" label="证券代码" sortable style="font-size: 14px" width="90" />
    <el-table-column prop="name" label="证券名称" sortable style="font-size: 14px" width="90" />
    <el-table-column prop="direction" label="交易类别" sortable style="font-size: 14px" width="90" />
    <el-table-column prop="num" label="交易数量" class-name="right-cell" style="font-size: 14px" width="70" />
    <el-table-column prop="price" label="成交价格" class-name="right-cell" style="font-size: 14px" width="70" />
    <el-table-column prop="amount" label="成交金额" sortable class-name="right-cell" style="font-size: 14px" width="90" />
<!--    <el-table-column prop="remark" label="备注" :formatter="remarkFormatter" style="font-size: 14px" width="160" />-->
    <el-table-column prop="remark" label="备注"  style="font-size: 14px" width="160" >
      <template #default="scope">
        <!-- 判断为编辑状态 -->
        <el-input
            v-if="
              state.tableRowEditIndex === scope.$index &&
              state.tableColumnEditIndex == scope.column.id
            "
            style="font-size: 14px;height: 22px"
            ref="tableRowInputRef"
            v-model="scope.row.remark"
            @keyup.enter="
              $event => {
                $event.target.blur()
              }
            "
            @blur="onInputTableBlur(scope)"
        />
        <!-- 判断为显示状态 -->
        <p v-else  @dblclick="dbClickCell(scope)">
          {{ scope.row.remark || '双击编辑'}}
        </p>
      </template>
    </el-table-column>>
  </el-table>
</template>

<style>
.el-table th {
  color: black; /* 表头字体为黑色 */
  text-align: center;
}
.el-table .cell{
  padding-left: 5px;
  padding-right: 3px;
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