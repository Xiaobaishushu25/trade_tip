<script setup lang="ts">
import {nextTick, onMounted, ref,Ref, watch,onBeforeUnmount,reactive} from "vue";
import {RowData, StockInfoG, StockLiveData} from "../type.ts";
import {invoke} from "@tauri-apps/api/core";
import {emit, listen} from "@tauri-apps/api/event";
import StockGroupMange from "./group/StockGroupMange.vue";
import {useRouter} from "vue-router";
import {store} from "../store.ts";
import {errorNotification, successNotification} from "../utils.ts";
import {WebviewWindow} from "@tauri-apps/api/webviewWindow";

// defineProps<{groupName: string, stocks:StockInfoG[]}>({
const props = defineProps({
  groupName: {
    type: String,
    required: true
  },
  stocks_change:{
    type: Boolean,
    required: true
  },
  activeName:{
    type: String,
    required: true
  }
})

const router = useRouter()
const table = ref()
const StockInfoGs = ref<StockInfoG[]>([]);
const nowSelectStock = ref<StockInfoG>();
const tableHeight = ref();
const showGroupManage = ref(false);
const contextMenuShow = ref(false)


const options = {
  theme: 'flat',
  name:"",
  code:"",
  zIndex: 3,
  x: 500,
  y: 200
}

watch(()=>store.isBlur,(newValue)=>{
  if (newValue){
    contextMenuShow.value = false;
  }
});

// watch(() => props.groupName, (newValue, oldValue) => {
//   console.log('groupName changed:', newValue, oldValue);
//   updateStockInfoG();
// });
watch(() => props.stocks_change, (_) => {
  console.log('分组内的的股票changed:', props.groupName);
  updateStockInfoG();
  if (props.activeName === props.groupName){
    // console.log("当前页面更新了，开始实时查询")
    invoke("query_live_stocks_data_by_group_name",{groupName:props.groupName}).catch(err => {
      console.log(err);
    })
  }
});
onMounted(() => {
  calculateTableHeight();
  updateStockInfoG();
  window.addEventListener('resize', calculateTableHeight);
  listen("live_stock_data", ({payload }) => {
    if (props.activeName == props.groupName){
      updateLiveData(payload);
    }
  });
  listen("delete_stock", ({payload }) => {
    console.log("删除了股票", payload)
    const index = StockInfoGs.value.findIndex(item => item.code == payload);
    if (index !== -1) {
      StockInfoGs.value.splice(index, 1);
    }
  });
})
listen("select-stock-detail", ({payload}) => {
  console.log(payload);
  StockInfoGs.value.find((item:StockInfoG) => {
    if (item.code === payload.code){
      clickRow(item,null);
      return true;
    }
  })
})
onBeforeUnmount(()=> {
  window.removeEventListener('resize', calculateTableHeight);
})
//用于更新数据
function updateStockInfoG() {
  invoke<StockInfoG[]>("query_stocks_by_group_name", {name: props.groupName}).then(res => {
    StockInfoGs.value = res;
    //这个函数每个tab里的table都会执行，所以要根据是否是当前选中的的tab来更新数据
    if (props.activeName===props.groupName){
      store.stockinfoGs = res;
    }
    StockInfoGs.value.forEach(item => {
      item.rowData = ref({
        code:item.code, box: ["","",undefined], change: "", ma: ["均线--","normal"], price: 0.0,
        breathClass:"", advise: ["具体分析","normal"]
      })
    })
  }).catch(err => {
    console.log(err);
  })
}
function getColor(percent:number){
  if (percent > 0) {
    return 'red'; // 红色
  } else if (percent < 0) {
    return 'green'; // 绿色
  } else {
    return 'black'; // 黑色
  }
}
function calculateTableHeight() {
  // 计算表格高度，假设您希望表格高度为视口高度减去 100px
  tableHeight.value = `${window.innerHeight - 100}px`;
}
function showContextMenu(row: StockInfoG, _: any, e: MouseEvent) {
  nowSelectStock.value = row;
  options.x = e.x;
  options.y = e.y;
  options.code = row.code;
  options.name = row.name;
  contextMenuShow.value=true

}
async function updateLiveData(live_data:Record<string, StockLiveData>){
  //遍历StockInfoGs
  if (props.activeName===props.groupName){
    for (let i = 0; i < StockInfoGs.value.length; i++) {
      let stock = StockInfoGs.value[i];
      let code = stock.code;
      if (live_data[code] != undefined){
        let rowData = stock.rowData;
        const newPrice = live_data[code].price;
        const oldPrice = rowData?.price ?? 0;
        stock.live_data = live_data[code];
        rowData.price = live_data[code].price;
        rowData.ma = judgeMaState(stock);
        rowData.box = computeBox(stock);
        rowData.advise = getAdvise(stock);
        if (store.stockinfoG?.code==code){ //为了在细节页面能够看见实时消息，需要更新全局状态的当前股票信息
          store.stockinfoG.live_data = live_data[code];
          store.stockinfoG.rowData = rowData;
        }
        // 根据价格变化设置呼吸灯效果
        if (newPrice > oldPrice) {
          await nextTick();
          rowData.breathClass = 'red-breath';
          setTimeout(() => {
            rowData.breathClass = 'no-breath';
          }, 1000);
        } else if (newPrice < oldPrice) {
          await nextTick();
          rowData.breathClass = 'green-breath';
          setTimeout(() => {
            rowData.breathClass = 'no-breath';
          }, 1000);
          // 价格降低，显示绿色呼吸灯
        } else {
          // 价格未变，不显示呼吸灯
          // rowDataMap.get(code)!.breathClass = 'green-breath';
          rowData.breathClass = 'no-breath';
        }
      }
    }
    //因为已经限定了当前tab就是选中的tab，所以这里同步更新全局状态的StockInfoGs，不然详情页拿不到数据
    store.stockinfoGs = StockInfoGs.value;
  }
}
const getRowClass=({
  row,
}:{row:StockInfoG,rowIndex:number})=>{
  {
    if (row.rowData.breathClass) {
      const breathClass = row.rowData.breathClass; // 使用非空断言，因为我们知道存在该code
      return {
        // 其他类名...
        [breathClass]: true, // 使用方括号语法动态绑定类名
      };
    } else {
      return {}; // 或者返回一个包含默认类名的对象
    }
  }
}
function removeStock(code: string){
  if (code.length > 0){
    invoke("remove_stock_from_group", {code: code,groupName: props.groupName}).then(_ => {
      // updateStockInfoG();直接移除就行了，别全部更新了吧
      // StockInfoG.value = StockInfoG.value.filter(item => item.code !== code);
      const index = StockInfoGs.value.findIndex(item => item.code == code);
      if (index !== -1) {
        StockInfoGs.value.splice(index, 1);
      }
    }).catch(err => {
      console.log(err)
    })
  }
}

function manageGroup(){
  showGroupManage.value = !showGroupManage.value;
}
function clickRow(row: StockInfoG, _: any){
  nowSelectStock.value = row;
  store.stockinfoG = nowSelectStock;
  // store.count = nowSelectStock.value.code
  router.push("/main/detail")
}

//根据股票code更新是否持有，更新为当前是否持有的反状态
//如果成功更新成功，更新该股票（当前行）的状态信息，还要通知"持有"分组股票有变化了
// 同时判断是否是持有标签页，如果是，要移除
function updateHold(){
  let code = nowSelectStock.value!.code;
  invoke("update_stock_hold", {code: code,hold: !nowSelectStock.value!.hold}).then(_ => {
    successNotification("更新成功");
    store.stockGroups.forEach((item) => {
      if (item.name=="持有"){
        item.stocks_change = !item.stocks_change
      }
    })
    const index = StockInfoGs.value.findIndex(item => item.code === code);
    if (index !== -1) {
      if (props.groupName=="持有"){
        StockInfoGs.value.splice(index, 1);
      }else {
        StockInfoGs.value[index].hold = !StockInfoGs.value[index].hold;
      }
    }
  }).catch(err => {
    console.log(err);
    errorNotification(err)
  })
}
// const successNotification = (content:string) => {
//   ElNotification({
//     title: 'Success',
//     message: content,
//     type: 'success',
//     position: 'bottom-right',
//   })
// }
// const errorNotification = (content:string) => {
//   ElNotification({
//     title: 'Error',
//     message: content,
//     type: 'error',
//     position: 'bottom-right',
//     duration: 0,
//   })
// }
function judgeMaState(stock:StockInfoG){
  if (stock.live_data == undefined){
    return ["---","normal"];
  }
  let ma5 = stock.live_data.ma5;
  let ma10 = stock.live_data.ma10;
  let ma20 = stock.live_data.ma20;
  if (ma5 > ma10 && ma10 > ma20) {
    // rowDataMap.get(stock.code)!.ma = "均线多头";
    return ["均线多头","up"];
  } else if (ma20 > ma10 && ma10 > ma5) {
    // rowDataMap.get(stock.code)!.ma = "均线空头";
    return ["均线空头","down"];
  } else {
    // rowDataMap.get(stock.code)!.ma = "均线缠绕";
    return ["均线缠绕","normal"];
  }
}
function getAdvise(stock: StockInfoG){
  // let rowData = rowDataMap.get(stock.code);
  let rowData = stock.rowData;
  if (rowData!=undefined){
    let box = rowData.box[0];
    if (rowData.ma[0]=="均线多头"){
      if (box=="已突破箱体"){
        if ((stock.live_data.price-rowData.box[2])>0.05*rowData.box[2]){
          // rowData.advise = ["积极持有","up-tag"];
          return ["积极持有","up-tag"];
        }else {
          // rowData.advise = ["买入","up-tag"];
          return ["买入","up-tag"];
        }
      }else if (box=="下轨区"){
        // rowData.advise = ["买入","up-tag"];
        return ["买入","up-tag"];
      }else if(box=="中轨区"){
        // rowData.advise = ["积极持有","up-tag"];
        return ["积极持有","up-tag"];
      }else if(box=="上轨区"){
        // rowData.advise = ["谨慎持有","normal-tag"];
        return ["谨慎持有","normal-tag"];
      } else {
        // rowData.advise = ["具体分析","normal"];
        return ["具体分析","normal"];
      }
    }else if (rowData.ma[0]=="均线空头"){
      if (box=="下轨区"||box=="已突破箱体"){
        // rowData.advise = ["谨慎买入","normal-tag"];
        return ["谨慎买入","normal-tag"];
      }else {
        // rowData.advise = ["卖出","down-tag"];
        return ["卖出","down-tag"];
      }
    }else if (rowData.ma[0]=="均线缠绕"){
      if (box=="下轨区"){
        // rowData.advise = ["买入","up-tag"];
        return ["买入","up-tag"];
      }else if (box=="已突破箱体"){
        if ((stock.live_data.price-rowData.box[2])>0.05*rowData.box[2]){
          // rowData.advise = ["积极持有","up-tag"];
          return ["积极持有","up-tag"];
        }
        // rowData.advise = ["买入","up-tag"];
        return ["买入","up-tag"];
      }
      else if (box=="上轨区"){
        // rowData.advise = ["谨慎卖出","normal"];
        return ["谨慎卖出","normal"];
      }else {
        // rowData.advise = ["具体分析","normal"];
        return ["具体分析","normal"];
      }
    }
  }
}
const filterAdvise = (value: string, stock: StockInfoG) => {
  if (value==="买入(未持有)"){
    return stock.rowData?.advise[0]==="买入"&&!stock.hold;
  }
  return value===stock.rowData?.advise[0];
}
const filterName = (value: string, stock: StockInfoG) => {
  if (value==="未持有"){
    return !stock.hold;
  }else {
    return true;
  }
}
function computeBox(stock: StockInfoG){
  let code = stock.code;
  let boxes = store.boxData[code];
  //打印code和对应的boxes
  if (boxes!=undefined&&stock.live_data?.price!=undefined){
    let box = comparePriceWithBox(stock.live_data.price, boxes);
    // rowDataMap.get(code)!.box = box[0];
    return box;
  }
  return ["----","normal",undefined];
}
function comparePriceWithBox(price: number,boxes:number[]): [string,string,number|undefined] {
  if (price < boxes[0]) {
    return ["已跌破箱体","down",undefined];
  } else if (price > boxes[boxes.length - 1]) {
    return ["已突破箱体","up",boxes[boxes.length - 1]];
  }
  for (let i = 0; i < boxes.length - 1; i++) {
    if (price >= boxes[i] && price <= boxes[i + 1]) {
      // const divideBox = `价格在${boxes[i].toFixed(3)}和${boxes[i + 1].toFixed(3)}之间`;
      return divideBox(price, boxes[i], boxes[i + 1]);
    }
  }
  return ["未知","normal",undefined];
}
function divideBox(price: number, down: number, up: number): [string,string,undefined] {
  const range = up - down;
  // const eachPart = range / 8.0;
  const eachPart = range / store.config.data_config.box_num;

  const lowerBound = down;
  const middleLowerBound = down + eachPart;
  const middleUpperBound = up - eachPart;
  if (price >= lowerBound && price <= middleLowerBound) {
    return ["下轨区","up",undefined];
  } else if (price > middleLowerBound && price <= middleUpperBound) {
    return ["中轨区","normal",undefined];
  } else if (price > middleUpperBound) {
    return ["上轨区","down",undefined];
  } else {
    return ["中轨区","normal",undefined];
  }
}
let isLoading = ref(false);
async function judgeCanT(){
  isLoading.value = true;
  let codes = StockInfoGs.value.map(stockInfo => stockInfo.code)
  invoke("judge_can_t", {codes: codes}).then(async res => {
    // 构建新的集合
    const combinedData = res.map(([code, trend]) => {
      const stock = StockInfoGs.value.find(stockInfo => stockInfo.code === code);
      return {
        code: code,
        name: stock ? stock.name : '', // 获取对应的name
        trend: trend // 从res获取的trend
      };
    });
    let jsonComponent = JSON.stringify(combinedData);
    localStorage.setItem('trendData', jsonComponent);
    // const combinedDataString = encodeURIComponent(jsonComponent);
    const webview = new WebviewWindow('cant', {
      url: '/#/cant',
      // url: `/#/cant?combinedData=${combinedDataString}`, // 传递 serialized combinedData
      center: true,
      title: '趋势判断',
      width: 330,
      height: 500,
      decorations: false,
      resizable: true,
      dragDropEnabled: false,
      visible: false,
      alwaysOnTop: true,
    });
    await webview.once('tauri://created', async function () {
      await webview.show()
    });
    isLoading.value = false;
    // await webview.show()
  }).catch(err => {
    isLoading.value = false;
    console.log(err);
    // errorNotification(err)
  })
}
</script>

<template>
  <div class="table-container">
      <el-table
          ref = "table"
          :height="tableHeight"
          :data="StockInfoGs"
          style="width: 100%;font-size: 14px"
          :max-height="tableHeight"
          :row-class-name="getRowClass"
          @row-dblclick="clickRow"
          @row-contextmenu="showContextMenu"
      >
        <el-table-column prop="code" label="代码" style="font-size: 14px" width="80" />
        <el-table-column
            prop="name"
            label="名称"
            width="180"
            :filters="[
        { text: '未持有', value: '未持有' },
      ]"
            :filter-method="filterName"
        >
          <template #default="scope">
            <el-text :style="{ color: scope.row.hold ?'orange':'black' }">{{scope.row.name}}</el-text>
          </template>
        </el-table-column>
        <el-table-column
            prop="live_data.price"
            label="现价"
            sortable>
          <template #default="scope">
            <el-text :style="{color:getColor(scope.row.live_data?.percent),fontSize:'15px'}">{{scope.row.live_data?.price}}</el-text>
          </template>
        </el-table-column>
        <el-table-column prop="live_data.percent" label="涨跌%" sortable >
          <template #default="scope">
            <el-text :style="{color:getColor(scope.row.live_data?.percent),fontSize:'15px',fontWeight:'bold'}">{{scope.row.live_data?.percent}}%</el-text>
          </template>
        </el-table-column>
        <el-table-column prop="price" label="均线状态">
          <template #default="scope">
<!--            <el-text :class="rowDataMap.get(scope.row.code).value[1]" >{{rowDataMap.get(scope.row.code).value[0]}}</el-text>-->
            <el-text :class="scope.row.rowData?.ma?.[1]" >{{scope.row.rowData?.ma?.[0]}}</el-text>
          </template>
        </el-table-column>
        <el-table-column prop="box" label="箱体">
          <template #default="scope">
<!--            <el-text :class="computeBox(scope.row)[1]" >{{computeBox(scope.row)[0]}}</el-text>-->
            <el-text :class="scope.row.rowData?.box?.[1]" >{{scope.row.rowData?.box?.[0]}}</el-text>
          </template>
        </el-table-column>
        <el-table-column
            prop="box"
            label="操作建议"
            :filters="[
        { text: '买入(未持有)', value: '买入(未持有)' },
        { text: '买入', value: '买入' },
        { text: '卖出', value: '卖出' },
      ]"
            :filter-method="filterAdvise"
        >
          <template #default="scope">
            <el-tag :class="scope.row.rowData?.advise?.[1]" >{{scope.row.rowData?.advise?.[0]}}</el-tag>
          </template>
        </el-table-column>
      </el-table>
    <context-menu
        v-model:show="contextMenuShow"
        :options="options"
    >
      <context-menu-item label="置顶(todo)" @click="" />
      <context-menu-item :label="nowSelectStock?.hold?'已清仓':'已持有'"  @click="updateHold()" />
      <context-menu-item label="从当前分组移除" @click="removeStock(options.code)" />
      <context-menu-sperator />
      <context-menu-item label="管理分组" @click="manageGroup()" />
<!--        <context-menu-item label="删除" @click="onMenuClick(2)" />-->
<!--        <context-menu-item label="Item2" @click="onMenuClick(3)" />-->
<!--      </context-menu-group>-->
    </context-menu>
    <el-button class="floating-button" plain :loading="isLoading"  @click="judgeCanT">T</el-button>
  </div>
  <StockGroupMange :name="options.name" :code="options.code" :show-dialog="showGroupManage"></StockGroupMange>
</template>

<style >
.table-container{
  min-height: calc(100vh - 100px);
  max-height: calc(100vh - 100px);
}
.up{
  color: red;
  font-weight: bold;
}
.down{
  color: green;
  font-weight: bold;
}
.normal{
  color: #000;
}
.up-tag{
  color: red;
  font-weight: bold;
  background-color: rgba(255, 0, 0, 0.2);
}
.down-tag{
  color: green;
  font-weight: bold;
  background-color: rgba(0, 128, 0, 0.2);
}
/* 红色呼吸灯效果 */
@keyframes redBreath {
  0% { background-color: rgba(255, 0, 0, 0.2); }
  50% { background-color: rgba(255, 0, 0, 0.3); }
  100% { background-color: rgba(255, 0, 0, 0.1); }
}

/* 绿色呼吸灯效果 */
@keyframes greenBreath {
  0% { background-color: rgba(0, 128, 0, 0.2); }
  50% { background-color: rgba(0, 128, 0, 0.3); }
  100% { background-color: rgba(0, 128, 0, 0.1); }
}

/* 初始状态不显示呼吸灯 */
.no-breath {
  background-color: inherit;
  animation: none;
}

/* 显示红色呼吸灯 */
.red-breath {
  animation: redBreath 1s infinite;
}

/* 显示绿色呼吸灯 */
.green-breath {
  animation: greenBreath 1s infinite;
}
.floating-button {
  position: fixed!important;
  z-index: 3000;
  bottom: 130px;
  right: 50px;
}
</style>