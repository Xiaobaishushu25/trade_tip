<script setup lang="ts">
import {TransactionRecord} from "../../type.ts";
import {ref,reactive, watch,onMounted,nextTick,} from "vue";
import {invoke} from "@tauri-apps/api/core";
import {emit, listen} from "@tauri-apps/api/event";

import {errorNotification} from "../../utils.ts";
import {store} from "../../store.ts";


const tableRef = ref(null);
const tableRowInputRef: any = ref(null);

const contextMenuShow = ref(false)
const options = {
  theme: 'flat',
  date:"",
  time:"",
  code:"",
  zIndex: 3,
  x: 500,
  y: 200
}

let defaultRemark:string[] = []
onMounted(async () => {
  invoke<TransactionRecord[]>('query_transaction_records',{}).then(data => {
    data.forEach(item => {if(item.remark==null){item.remark = ''}})//重要，防止remark为null导致的的不提示问题
    addRecords(data);
  }).catch(e => {
    console.error(e)
  });
  const storedObjectString = localStorage.getItem('config');
  const myObjectFromStorage = JSON.parse(storedObjectString);
  defaultRemark = myObjectFromStorage.display_config.default_remark.map(item => {
    return {"value":item}
  });
  await listen("disPlay_update", (data)=>{
    defaultRemark = data.payload.default_remark.map(item => {
      return {"value":item}
    });
  });
  await listen("update_record_event", (data)=>{
    let payload = data.payload;
    let changeRecord = transactionRecords.find(record => record.date === payload.date && record.time === payload.time && record.code === payload.code);
    changeRecord.remark = payload.remark;
    filteredRecords.value = transactionRecords;
    if(selectedCode!='0'){
      codeFilter(selectedCode);
    }
  });
})
let transactionRecords: TransactionRecord[] = [];
const filteredRecords = ref(transactionRecords);
let selectedCode = '0';

const querySearch = (queryString: string, cb: any) => {
  const results = queryString
    ? defaultRemark.filter(item =>item.value.includes(queryString))
    : defaultRemark
  cb(results)
}

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
  selectedCode = specifiedCode;
  filteredRecords.value = transactionRecords.filter(record => record.code === specifiedCode);
}
async function deleteAllRecords() {
  transactionRecords = [];
  filteredRecords.value = transactionRecords;
  selectedCode = '0';
}
async function addRecords(records: TransactionRecord[]) {
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
  store.isEditingRecord = true
  // 找到单个格子独有的属性 - 这里是用所在行跟列id区别显示
  state.tableRowEditIndex = scope.$index
  state.tableColumnEditIndex = scope.column.id
  nextTick(() => {
    if (tableRowInputRef.value) {
      // 获取焦点
      tableRowInputRef.value.focus();
      // done 如何主动触发提示内容,之前问题是在输入框获得焦点后，不主动触发提示内容。
      //使用tableRowInputRef.value.suggestions = defaultRemark;强制显示提示后，只会显示一瞬间，然后马上关闭
      //后来发现问题在于remark的内容是null导致的。
      // 手动设置建议内容并强制显示下拉框
      // tableRowInputRef.value.suggestions = defaultRemark;
      // 模拟输入触发提示
      // simulateInput();
      // showAutocompleteSuggestions(tableRowInputRef.value);
    }
    // 获取焦点
  })
}

// 表格中input取消焦点,select变化
const onInputTableBlur = async (scope: any) => {
  store.isEditingRecord = false
  state.tableRowEditIndex = undefined
  state.tableColumnEditIndex = undefined
  let data = scope.row;
  if (data.remark !== '双击编辑') {
    let result = await invoke('update_transaction_record',{record: data});
    if (result!=null) {
      errorNotification(`更新备注失败：${result}`)
    }else {
      //把这个消息发给蜡烛图，让他更新买卖点
      await emit('update_record_event', data)
    }
  }
}

function showContextMenu(row: TransactionRecord, _: any, e: MouseEvent) {
  options.x = e.x;
  options.y = e.y;
  options.date = row.date;
  options.time = row.time;
  options.code = row.code;
  contextMenuShow.value=true
}
async function deleteRecord() {
  invoke('delete_transaction_record_by_primary', {date: options.date, time: options.time, code: options.code}).then(data => {
    console.log("删除成功")
    transactionRecords = transactionRecords.filter(record => record.date !== options.date || record.time !== options.time || record.code !== options.code)
    filteredRecords.value = transactionRecords
  }).catch(e => {
    console.error(e)
    errorNotification("删除失败")
  })
}

defineExpose({ codeFilter, deleteAllRecords, addRecords})
</script>

<template>
  <el-table
      ref="tableRef"
      :data="filteredRecords"
      :row-class-name="tableRowClassName"
      max-height="calc(100vh - 80px)"
      @row-contextmenu="showContextMenu"
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
    <el-table-column prop="remark" label="备注"  style="font-size: 14px" width="150" >
      <template #default="scope">
        <!-- 判断为编辑状态 -->
        <el-autocomplete
            v-if="
            state.tableRowEditIndex === scope.$index &&
            state.tableColumnEditIndex == scope.column.id
            "
            style="font-size: 13px; height: 22px"
            ref="tableRowInputRef"
            :trigger-on-focus="true"
            v-model="scope.row.remark"
            :fetch-suggestions="querySearch"
            placeholder="请输入"
            class="custom-autocomplete"
            @keyup.enter="
              $event => {
                $event.target.blur()
              }
            "
            @blur="onInputTableBlur(scope)"
          />
<!--        <el-input-->
<!--            v-if="-->
<!--              state.tableRowEditIndex === scope.$index &&-->
<!--              state.tableColumnEditIndex == scope.column.id-->
<!--            "-->
<!--            style="font-size: 14px;height: 22px"-->
<!--            ref="tableRowInputRef"-->
<!--            v-model="scope.row.remark"-->
<!--            @keyup.enter="-->
<!--              $event => {-->
<!--                $event.target.blur()-->
<!--              }-->
<!--            "-->
<!--            @blur="onInputTableBlur(scope)"-->
<!--        />-->
        <!-- 判断为显示状态 -->
        <p v-else  @dblclick="dbClickCell(scope)">
          {{ scope.row.remark || '双击编辑'}}
        </p>
      </template>
    </el-table-column>>
  </el-table>
  <context-menu
      v-model:show="contextMenuShow"
      :options="options"
  >
    <context-menu-item label="删除(直接!)" @click.left="deleteRecord" />
  </context-menu>
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

.custom-autocomplete .el-input {
  --el-input-focus-border-color: grey!important; /* 上 右 下 左 */
}
.custom-autocomplete .el-input__wrapper {
  padding: 0 0 0 2px!important; /* 上 右 下 左 */
}
</style>
