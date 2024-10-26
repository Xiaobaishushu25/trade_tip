<script setup lang="ts">
import {WebviewWindow} from "@tauri-apps/api/webviewWindow";
import {saveWindowState, StateFlags} from "@tauri-apps/plugin-window-state";
import { ref, computed, onMounted } from 'vue';
import { useRoute } from 'vue-router';
import {emit} from "@tauri-apps/api/event";


// 接收路由参数
// const router = useRouter();
// const route = useRoute(); // 使用 useRoute 获取当前路由信息，注意，是 useRoute 而不是 useRouter。useRouter 用于导航，而 useRoute 用于访问当前路由的信息
// const combinedData = ref(route.query.combinedData || ''); // 从查询参数中获取数据

// 用于存储表格数据
const tableData = ref([]);

// // 计算属性，解析 combinedData
// const parsedCombinedData = computed(() => {
//   try {
//     let data = JSON.parse(decodeURIComponent(combinedData.value)); // 解析为原始数据格式
//     return data; // 返回解析后的数据
//   } catch (error) {
//     console.error('Failed to parse combinedData:', error);
//     return null; // 或者返回一个默认值
//   }
// });

// 在组件挂载后更新表格数据
onMounted(() => {
  const storedObjectString = localStorage.getItem('trendData');
  console.log(storedObjectString);
  const myObject = JSON.parse(storedObjectString);
  tableData.value = myObject;
  // if (parsedCombinedData.value) {
  //   tableData.value = parsedCombinedData.value; // 更新 tableData
  // }
});

const tableRowClassName = ({row}: {
  row: { code: string; name: string; trend: string; },
}) => {
  if (row.trend.includes('up')){
    return 'red-row'
  }else if (row.trend.includes('down')){
    return 'green-row'
  }
  return 'black-row'
}
async function clickRow(row: { code: string; name: string; trend: string; }, _: any){
  await emit('select-stock-detail', {code: row.code});
}
async function window_minimize(){
  await WebviewWindow.getCurrent().minimize()
}
async function window_close(){
  await saveWindowState(StateFlags.ALL);
  await WebviewWindow.getCurrent().close();
}
</script>

<template>
  <div>
  <div data-tauri-drag-region class="titlebar">
    <img src="../assets/icon.png" width="25" height="25" alt="Logo Image" style="margin-left: 5px;margin-right: 10px;user-select: none">
    <label style="font-family: 'Segoe UI'">趋势</label>
    <div id="stage-button">
      <inline-svg src="../assets/svg/minimize.svg" class="window-button min" @click.left="window_minimize"></inline-svg>
      <inline-svg src="../assets/svg/close.svg" class="window-button close"  @click.left="window_close"></inline-svg>
    </div>
  </div>
  <el-table
      ref="tableRef"
      :data="tableData"
      :row-class-name="tableRowClassName"
      @row-dblclick="clickRow"
      max-height="calc(100vh - 40px)"
      style="width: 100%;font-size: 14px;padding-left: 20px"
  >
    <el-table-column prop="code" label="证券代码"  style="font-size: 14px" width="90" >
      <template #default="scope">
        <span  class="no-hover">
          {{scope.row.code}}
        </span>
      </template>
    </el-table-column>
    <el-table-column prop="name" label="证券名称"  style="font-size: 14px" width="130" >
      <template #default="scope">
        <span  class="no-hover">
          {{scope.row.name}}
        </span>
      </template>
    </el-table-column>
    <el-table-column prop="trend" label="趋势"  style="font-size: 14px" width="80" >
      <template #default="scope">
        <span v-if="scope.row.trend.includes('up')" style="display: flex; align-items: center;">
          <inline-svg src="../assets/svg/up.svg" style="height: 18px;user-select: none"></inline-svg>
        </span>
        <span v-if="scope.row.trend.includes('down')" style="display: flex; align-items: center;">
          <inline-svg src="../assets/svg/down.svg" style="height: 18px;user-select: none"></inline-svg>
        </span>
        <span v-if="scope.row.trend.includes('normal')" class="no-hover">
          正常
        </span>
      </template>
    </el-table-column>
  </el-table>
  </div>
</template>

<style>
.el-table .green-row .cell {
  color: #0bd60b; /* 蓝色文本 */
}

</style>
<style scoped>
.no-hover {
  pointer-events: none; /* 禁用鼠标事件 */
  cursor: default; /* 使用默认光标 */
}

</style>