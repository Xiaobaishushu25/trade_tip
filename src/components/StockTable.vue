<script setup lang="ts">
import {nextTick, onMounted, ref, watch} from "vue";
import {StockInfoG} from "../type.ts";
import {invoke} from "@tauri-apps/api/core";
import {webviewWindow} from "@tauri-apps/api";
import {TauriEvent} from "@tauri-apps/api/event";
import StockGroupMange from "./group/StockGroupMange.vue";
import {useRouter} from "vue-router";

// defineProps<{groupName: string, stocks:StockInfoG[]}>({
const props = defineProps({
  groupName: {
    type: String,
    required: true
  },
  stocks_change:{
    type: Boolean,
    required: true
  }
  // stock_codes: {
  //   type: Array as PropType<string[]>,
  //   required: true
  // }
  // stocks: {
  //   type: Array as PropType<StockInfoG[]>,
  //   required: true
  // }
})
//
const router = useRouter()
const table = ref()
const show = ref(false)
const StockInfoG = ref<StockInfoG[]>([]);
const nowSelectStock = ref<StockInfoG>();
const tableHeight = ref();
const showGroupManage = ref(false);
const options = {
  // theme: 'win10 dark',
  name:"",
  code:"",
  zIndex: 3,
  x: 500,
  y: 200
}
// watch(groupName, (newValue, oldValue) => {
//   console.log(newValue, oldValue)
// })
watch(() => props.groupName, (newValue, oldValue) => {
  console.log('groupName changed:', newValue, oldValue);
  updateStockInfoG();
  // invoke<StockInfoG[]>("query_stocks_by_group_name", {name: newValue}).then(res => {
  //   console.log(res);
  //   StockInfoG.value = res;
  //   console.log(StockInfoG.value)
  // }).catch(err => {
  //   console.log(err);
  // })
});
watch(() => props.stocks_change, (_) => {
  console.log('分组的股票changed:', props.groupName);
  updateStockInfoG();
  // invoke<StockInfoG[]>("query_stocks_by_group_name", {name: newValue}).then(res => {
  //   console.log(res);
  //   StockInfoG.value = res;
  //   console.log(StockInfoG.value)
  // }).catch(err => {
  //   console.log(err);
  // })
});
onMounted(() => {
  console.log(props.groupName)
  updateStockInfoG();
  window.addEventListener('resize',getHeight)
  window.addEventListener('blur', ()=>{
    console.log("窗口失去焦点",show.value)
    show.value = false;
  })
  // webviewWindow.getCurrent().listen("WINDOW_BLUR", ({ event, payload }) => {
  // webviewWindow.getCurrent().listen(TauriEvent.WINDOW_BLUR, ({ event, payload }) => {
  //   console.log(event,payload)
  //   console.log("窗口失去焦点前",show.value)
  //   console.log("窗口失去焦点后",show.value)
  // });
})
//用于更新数据
function updateStockInfoG() {
  invoke<StockInfoG[]>("query_stocks_by_group_name", {name: props.groupName}).then(res => {
    StockInfoG.value = res;
  }).catch(err => {
    console.log(err);
  })
}
async function getHeight(){
  await nextTick();
  // window.innerHeight 浏览器窗口的可见高度，下面的 220 是除了table最大高度的剩余空间。
  console.log(1111,window.innerHeight);
  console.log(tableHeight.value)
  tableHeight.value = window.innerHeight  - 220;
  // tableHeight.value = window.innerHeight - table.value.offsetHeight - 220;
}
function showContextMenu(row: StockInfoG, _: any, e: MouseEvent) {
  nowSelectStock.value = row;
  console.log(row)
  console.log("当前选择的股票de value",nowSelectStock.value)
  // console.log(row, column, e,show.value)
  options.x = e.x;
  options.y = e.y;
  options.code = row.code;
  options.name = row.name;
  show.value=true

}
watch(() => show.value, (newValue, oldValue) => {
  console.log("show的值变换了"+newValue, oldValue)
})

function removeStock(code: string){
  if (code.length > 0){
    invoke("remove_stock_from_group", {code: code,groupName: props.groupName}).then(_ => {
      // updateStockInfoG();直接移除就行了，别全部更新了吧
      // StockInfoG.value = StockInfoG.value.filter(item => item.code !== code);
      const index = StockInfoG.value.findIndex(item => item.code == code);
      console.log(index)
      if (index !== -1) {
        StockInfoG.value.splice(index, 1);
      }
    }).catch(err => {
      console.log(err)
    })
  }
}
function computeBox(box:string|undefined){
  if (box==undefined){
    return "----";
  }else {
    return box.substring(0, 2);
  }
}

function manageGroup(){
  console.log("展示管理分组")
  showGroupManage.value = !showGroupManage.value;
  // showGroupManage.value = false;
}
function openChart(code:string){
  router.push("/newCandleChart")
  // router.back()
}
//根据股票code更新是否持有，更新为当前是否持有的反状态
//如果成功更新成功，更新该股票（当前行）的状态信息，同时判断是否是持有标签页，如果是，要移除
function updateHold(){
  let code = nowSelectStock.value!.code;
  invoke("update_stock_hold", {code: code,hold: !nowSelectStock.value!.hold}).then(_ => {
    const index = StockInfoG.value.findIndex(item => item.code === code);
    if (index !== -1) {
      if (props.groupName=="持有"){
        StockInfoG.value.splice(index, 1);
      }else {
        StockInfoG.value[index].hold = !StockInfoG.value[index].hold;
      }
    }
  }).catch(err => {
    console.log(err)
  })
}
// watch(showGroupManage,(newValue,oldValue)=>{
//   if (newValue){
//     console.log("值变换了 展示管理分组")
//   }else{
//     console.log("值变换了 隐藏管理分组")
//   }
// })
</script>

<template>
  <div class="container">
      <el-table
          ref = "table"
          :height="tableHeight"
          :data="StockInfoG"
          style="width: 100%"
          :max-height="tableHeight"
          @row-contextmenu="showContextMenu"
      >
        <el-table-column prop="code" label="代码" width="80" />
        <el-table-column prop="name" label="名称" width="180">
          <template #default="scope">
            <el-text :style="{ color: scope.row.hold ?'orange':'black' }">{{scope.row.name}}</el-text>
          </template>
        </el-table-column>
        <el-table-column prop="price" label="现价" />
        <el-table-column prop="change_percent" label="涨跌%" />
        <el-table-column prop="price" label="均线状态">
          <template #default="scope">
            <el-text >{{computeBox(scope.row.box)}}</el-text>
          </template>
        </el-table-column>
        <el-table-column prop="box" label="箱体">
          <template #default="scope">
            <el-text style="cursor: pointer" @click="openChart(scope.row.code)" >{{computeBox(scope.row.box)}}</el-text>
          </template>
        </el-table-column>
      </el-table>
    <context-menu
        v-model:show="show"
        :options="options"
    >
      <context-menu-item label="置顶" @click="" />
      <context-menu-item :label="nowSelectStock?.hold?'清仓':'持有'"  @click="updateHold()" />
      <context-menu-item label="提醒" @click="removeStock(options.code)" />
      <context-menu-sperator />
      <context-menu-item label="管理分组" @click="manageGroup()" />
<!--      <context-menu-group label="Menu with child">-->
<!--        <context-menu-item label="删除" @click="onMenuClick(2)" />-->
<!--        <context-menu-item label="Item2" @click="onMenuClick(3)" />-->
<!--      </context-menu-group>-->
    </context-menu>
  </div>

  <StockGroupMange :name="options.name" :code="options.code" :show-dialog="showGroupManage"></StockGroupMange>
</template>

<style scoped>

</style>