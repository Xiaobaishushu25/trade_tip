<script setup lang="ts">
import {nextTick, onMounted, ref, watch,onBeforeUnmount,reactive} from "vue";
import {rowData, StockInfoG, StockLiveData} from "../type.ts";
import {invoke} from "@tauri-apps/api/core";
import {listen} from "@tauri-apps/api/event";
import StockGroupMange from "./group/StockGroupMange.vue";
import {useRouter} from "vue-router";
import {store} from "../store.ts";
import {errorNotification, successNotification} from "../utils.ts";

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
// const rowDataMap:Map<string,rowData> = new Map();
const rowDataMap = reactive<Map<string,rowData>>(new Map()); // 使用reactive创建响应式Map
const tableHeight = ref();
const showGroupManage = ref(false);
const contextMenuShow = ref(false)


const options = {
  // theme: 'win10 dark',
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
watch(
    () => Array.from(rowDataMap.entries()),
    (newEntries) => {
      console.log("rowdatamaopchange了",rowDataMap)
      if (props.activeName===props.groupName){
        store.rowData.clear(); // 清空旧的Map数据
        newEntries.forEach(([key, value]) => {
          store.rowData.set(key, value);
        });
        console.log("此时的rowdatamap",store.rowData)
      }
    },
    { deep: true } // 因为我们关心Map内部的变化
);
// watch(groupName, (newValue, oldValue) => {
//   console.log(newValue, oldValue)
// })

// watch(() => props.groupName, (newValue, oldValue) => {
//   console.log('groupName changed:', newValue, oldValue);
//   updateStockInfoG();
// });
watch(() => props.stocks_change, (_) => {
  console.log('分组内的的股票changed:', props.groupName);
  updateStockInfoG();
  if (props.activeName === props.groupName){
    console.log("当前页面更新了，开始实时查询")
    invoke("query_live_stocks_data",{groupName:props.groupName}).catch(err => {
      console.log(err);
    })
  }
});
onMounted(() => {
  console.log(props.groupName)
  calculateTableHeight();
  updateStockInfoG();
  // window.addEventListener('resize',getHeight)
  // window.addEventListener('blur', ()=>{
  //   show.value = false;
  // })
  window.addEventListener('resize', calculateTableHeight);
  // invoke("query_live_stocks_data",{groupName:props.groupName}).catch(err => {
  //   console.log(err);
  // })
  listen("live_stock_data", ({payload }) => {
    if (props.activeName == props.groupName){
      console.log("收到实时数据",payload);
      updateLiveData(payload);
    }
  })
  // webviewWindow.getCurrent().listen("WINDOW_BLUR", ({ event, payload }) => {
  // webviewWindow.getCurrent().listen(TauriEvent.WINDOW_BLUR, ({ event, payload }) => {
  //   console.log(event,payload)
  //   console.log("窗口失去焦点前",show.value)
  //   console.log("窗口失去焦点后",show.value)
  // });
})
onBeforeUnmount(()=> {
  window.removeEventListener('resize', calculateTableHeight);
})
// beforeDestroy() {
//   window.removeEventListener('resize', this.calculateTableHeight);
// },
//用于更新数据
function updateStockInfoG() {
  invoke<StockInfoG[]>("query_stocks_by_group_name", {name: props.groupName}).then(res => {
    StockInfoGs.value = res;
    //这个函数每个tab里的table都会执行，所以要根据是否是当前选中的的tab来更新数据
    if (props.activeName===props.groupName){
      store.stockinfoGs = res;
    }
    rowDataMap.clear();
    // store.stockinfoGs.forEach(item => {
    res.forEach(item => {
      rowDataMap.set(item.code,{
        code:item.code, box: ["",undefined], change: "", ma: "", price: 0.0,
        // code:item.code, box: "", change: "", ma: "", price: 0.0,
        breathClass:"", advise: ["",""]
      });
    });
    console.log("给全局状态赋值",store.stockinfoGs)
    // console.log("初始化的",rowDataMap)
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
// async function getHeight(){
//   await nextTick();
//   // window.innerHeight 浏览器窗口的可见高度，下面的 220 是除了table最大高度的剩余空间。
//   console.log(1111,window.innerHeight);
//   console.log(tableHeight.value)
//   tableHeight.value = window.innerHeight  - 220;
//   // tableHeight.value = window.innerHeight - table.value.offsetHeight - 220;
// }
function calculateTableHeight() {
  // 计算表格高度，假设您希望表格高度为视口高度减去 100px
  tableHeight.value = `${window.innerHeight - 100}px`;
}
function showContextMenu(row: StockInfoG, _: any, e: MouseEvent) {
  nowSelectStock.value = row;
  // console.log("当前选择的股票de value",nowSelectStock.value)
  // console.log(row, column, e,show.value)
  options.x = e.x;
  options.y = e.y;
  options.code = row.code;
  options.name = row.name;
  contextMenuShow.value=true

}
// watch(() => show.value, (newValue, oldValue) => {
//   console.log("show的值变换了"+newValue, oldValue)
// })
async function updateLiveData(live_data:Record<string, StockLiveData>){
  console.log("收到实时数据",live_data);
  if (props.activeName === props.groupName){
    //遍历StockInfoGs
    for (let i = 0; i < StockInfoGs.value.length; i++) {
      const element = StockInfoGs.value[i];
      let code = element.code;
      if (live_data[code] != undefined){
        const newPrice = live_data[code].price;
        const oldPrice = rowDataMap.get(code)!.price;
        StockInfoGs.value[i].live_data = live_data[code];
        if (store.stockinfoG?.code==code){ //为了在细节页面能够看见实时消息，需要更新全局状态的当前股票信息
          store.stockinfoG.live_data = live_data[code];
        }
        rowDataMap.get(code)!.price = live_data[code].price;
        // 根据价格变化设置呼吸灯效果
        if (newPrice > oldPrice) {
          await nextTick();
          rowDataMap.get(code)!.breathClass = 'red-breath';
          setTimeout(() => {
            rowDataMap.get(code)!.breathClass = 'no-breath';
          }, 1000);
        } else if (newPrice < oldPrice) {
          await nextTick();
          rowDataMap.get(code)!.breathClass = 'green-breath';
          setTimeout(() => {
            rowDataMap.get(code)!.breathClass = 'no-breath';
          }, 1000);
          // 价格降低，显示绿色呼吸灯
        } else {
          // 价格未变，不显示呼吸灯
          // rowDataMap.get(code)!.breathClass = 'green-breath';
          rowDataMap.get(code)!.breathClass = 'no-breath';
        }
      }
    }
  }
}
// function getRowClass(row:StockInfoG){
//   console.log("get是",rowDataMap.get(row.code)!.breathClass);
//   return {
//     // 其他类名...
//     [rowDataMap.get(row.code)!.breathClass]: true, // 使用方括号语法动态绑定类名
//   };
// }
const getRowClass=({
  row,
}:{row:StockInfoG,rowIndex:number})=>{
  {
    // 假设rowDataMap是一个响应式引用或全局可访问的变量
    // 检查rowDataMap是否包含当前行的code
    // console.log("检查了吗样式",rowDataMap,row.code)
    if (rowDataMap.has(row.code)) {
      const breathClass = rowDataMap.get(row.code)!.breathClass; // 使用非空断言，因为我们知道存在该code
      return {
        // 其他类名...
        [breathClass]: true, // 使用方括号语法动态绑定类名
      };
    } else {
      // rowDataMap尚未初始化或当前行的code不存在，返回一个空对象或默认类名
      console.log("rowDataMap尚未包含", row.code);
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
// function getAdvise(stock: StockInfoG){
//   let rowData = rowDataMap.get(stock.code);
//   if (rowData!=undefined){
//     if (rowData.ma!="均线空头"&&(rowData.box=="下轨区"||rowData.box=="已突破箱体")){
//       rowDataMap.get(stock.code)!.advise = ["买入","up-tag"];
//       return ["买入","up-tag"]
//     }else if (rowData.ma=="均线空头"&&(rowData.box=="上轨区"||rowData.box=="已跌破箱体")){
//       rowDataMap.get(stock.code)!.advise = ["卖出","down-tag"];
//       return ["卖出","down-tag"]
//     }else {
//       rowDataMap.get(stock.code)!.advise = ["具体分析","normal"];
//       return ["具体分析","normal"]
//     }
//   }
//   // rowDataMap.get(stock.code)?.advise = ["----","normal"];
//   return ["----","normal"];
// }
async function getAdvise(stock: StockInfoG){
  let rowData = rowDataMap.get(stock.code);
  if (props.activeName==props.groupName){
    if (rowData!=undefined){
      await nextTick();
      console.log("获取建议",rowData.ma,rowData.box);
      if (rowData.ma!="均线空头"&&(rowData.box[0]=="下轨区"||rowData.box[0]=="已突破箱体")){
        rowDataMap.get(stock.code)!.advise = ["买入","up-tag"];
        return ["买入","up-tag"];
      }else if (rowData.ma=="均线空头"&&(rowData.box[0]=="上轨区"||rowData.box[0]=="已跌破箱体")){
        rowDataMap.get(stock.code)!.advise = ["卖出","down-tag"];
        return ["卖出","down-tag"];
      }else {
        rowDataMap.get(stock.code)!.advise = ["具体分析","normal"];
        return ["具体分析","normal"];
      }
    }
  }
  // rowDataMap.get(stock.code)?.advise = ["----","normal"];
  return ["----","normal"];
}
// function getAdvise(stock: StockInfoG){
//   let rowData = rowDataMap.get(stock.code);
//   if (rowData!=undefined){
//     console.log("获取建议")
//     // let box = rowData.box[0];
//     if (rowData.ma!="均线空头"&&(rowData.box[0]=="下轨区"||rowData.box[0]=="已突破箱体")){
//       // if (box==="已突破箱体"){
//       //   //计算现价和箱体之间的差值，如果差值大于5%，则认为已突破箱体很多，持有就好。
//       //   if ((stock.live_data!.price-rowData.box[1])>0.05*rowData.box[1]){
//       //     rowDataMap.get(stock.code)!.advise = ["积极持有","up-tag"];
//       //     return ["积极持有","up-tag"]
//       //   }
//       // }
//       rowDataMap.get(stock.code)!.advise = ["买入","up-tag"];
//       return ["买入","up-tag"]
//     }else if (rowData.ma=="均线空头"&&(rowData.box[0]=="上轨区"||rowData.box[0]=="已跌破箱体")){
//       rowDataMap.get(stock.code)!.advise = ["卖出","down-tag"];
//       return ["卖出","down-tag"]
//     }else {
//       rowDataMap.get(stock.code)!.advise = ["具体分析","normal"];
//       return ["具体分析","normal"]
//     }
//   }
//   // rowDataMap.get(stock.code)?.advise = ["----","normal"];
//   return ["----","normal"];
// }
const filterAdvise = (value: string, stock: StockInfoG) => {
  if (value==="买入(未持有)"){
    return rowDataMap.get(stock.code)!.advise[0]==="买入"&&!stock.hold;
  }
  return value===rowDataMap.get(stock.code)!.advise[0];
}
function computeBox(stock: StockInfoG){
  let code = stock.code;
  let boxes = store.boxData[code];
  //打印code和对应的boxes
  if (boxes!=undefined&&stock.live_data?.price!=undefined){
    let box = comparePriceWithBox(stock.live_data.price, boxes);
    // console.log("计算箱体",code,box);
    rowDataMap.get(code)!.box = [box[0],box[2]];
    console.log("计算箱体",code,rowDataMap.get(code));
    // rowDataMap.get(code)!.advise = ["买入","up-tag"];
    // rowDataMap.get(code)!.box = box[0];
    return box;
  }
  return ["----","normal"];
}

function manageGroup(){
  console.log("展示管理分组")
  showGroupManage.value = !showGroupManage.value;
  // showGroupManage.value = false;
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
  console.log("判断均线状态")
  if (stock.live_data == undefined){
    return ["---","normal"];
  }
  let ma5 = stock.live_data.ma5;
  let ma10 = stock.live_data.ma10;
  let ma20 = stock.live_data.ma20;
  if (ma5 > ma10 && ma10 > ma20) {
    rowDataMap.get(stock.code)!.ma = "均线多头";
    return ["均线多头","up"];
  } else if (ma20 > ma10 && ma10 > ma5) {
    rowDataMap.get(stock.code)!.ma = "均线空头";
    return ["均线空头","down"];
  } else {
    rowDataMap.get(stock.code)!.ma = "均线缠绕";
    return ["均线缠绕","normal"];
  }
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
  const eachPart = range / 5.0;

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
        <el-table-column prop="name" label="名称" width="180">
          <template #default="scope">
            <el-text :style="{ color: scope.row.hold ?'orange':'black' }">{{scope.row.name}}</el-text>
          </template>
        </el-table-column>
        <el-table-column prop="live_data.price" label="现价" sortable>
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
<!--            <el-text :style="{color:judgeMaState(scope.row)[1],fontSize:'15px',fontWeight:'bold'}" >{{judgeMaState(scope.row)[0]}}</el-text>-->
            <el-text :class="judgeMaState(scope.row)[1]" >{{judgeMaState(scope.row)[0]}}</el-text>
          </template>
        </el-table-column>
        <el-table-column prop="box" label="箱体">
          <template #default="scope">
            <el-text :class="computeBox(scope.row)[1]" >{{computeBox(scope.row)[0]}}</el-text>
          </template>
        </el-table-column>
        <el-table-column
            label="操作建议"
            :filters="[
        { text: '买入(未持有)', value: '买入(未持有)' },
        { text: '买入', value: '买入' },
        { text: '卖出', value: '卖出' },
      ]"
            :filter-method="filterAdvise"
        >
          <template #default="scope">
<!--            <div v-let="{ text, styleClass } = getAdvise(scope.row)">-->
<!--              <el-text :class="styleClass" >{{text}}</el-text>-->
<!--            </div>-->
<!--            <el-text :class="getAdvise(scope.row)[1]" >{{getAdvise(scope.row)[0]}}</el-text>-->
<!--            <el-tag  >测试</el-tag>-->
            <el-tag :class="getAdvise(scope.row)[1]" >{{getAdvise(scope.row)[0]}}</el-tag>
          </template>
        </el-table-column>
      </el-table>
    <context-menu
        v-model:show="contextMenuShow"
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
</style>