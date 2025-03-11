<script setup lang="ts">
import {store} from "../../store.ts";
import { ref, watch} from "vue";
import {invoke} from "@tauri-apps/api/core";
import RTable from "../transactionRecordComponents/RTable.vue";
import StockGroupMange from "../group/StockGroupMange.vue";


const ImageSrc = ref('');
const rTableRef = ref();
const IntradayChartShow = ref(false);
const utilShow = ref(false); //工具窗口是否展示

const showGroupManage = ref(false);

const utilPrice1 = ref(store.stockinfoG?.live_data?.price); //价格1
const utilPrice2 = ref(); //价格2
const utilLoss = ref(0); //止损额度
const utilChange = ref(); //价格涨跌百分比
const utilTotalAmount = ref(); //建议买入总金额
const utilTotalShare = ref(); //建议买入总手数

watch(() => store.stockinfoG!.code, async (newVal) => {
  // console.log('分组内的的股票changed:', newVal);
  if (IntradayChartShow.value){
    await getIntradayChartImg(newVal);
    rTableRef.value?.codeFilter(newVal);
  }
  utilPrice1.value = store.stockinfoG?.live_data?.price;
});
watch(IntradayChartShow, async (newVal) => {
  if (newVal){
    console.log("打开分时图");
    await getIntradayChartImg(store.stockinfoG!.code);
    rTableRef.value?.codeFilter(store.stockinfoG!.code);
  }
})
function manageGroup(){
  showGroupManage.value = !showGroupManage.value;
}
function getColor(percent:number){
  if (percent > 0) {
    return 'red'; // 红色
  } else if (percent < 0) {
    return '#00CD00'; // 绿色
  } else {
    return 'black'; // 黑色
  }
}
async function getIntradayChartImg(code:string){
  invoke<ArrayBuffer>("query_intraday_chart_img", {code: code}).then(data => {
    const blob = new Blob([data], { type: 'image/png' }); // 根据你的图片类型设置MIME类型，如'image/jpeg'、'image/png'等
    ImageSrc.value = URL.createObjectURL(blob);
  }).catch(err => {
    ImageSrc.value = '';
    console.log(err)
  })
}

function updateUtil(){
  calculateChange();
  calculateTotalAmount();
  calculateTotalShare();
}
// 计算价格涨跌百分比
function calculateChange() {
  if (utilPrice1.value === undefined || utilPrice1.value === null) {
    return;
  }
  if (utilPrice1.value !== 0 && utilPrice2.value !== 0) {
    utilChange.value = (((utilPrice2.value - utilPrice1.value) / utilPrice1.value) * 100).toPrecision(2);
  }
}

// 计算买入总金额
function calculateTotalAmount() {
  console.log("计算总额度")
  //打印utilChange
  console.log((Math.abs(utilChange.value) / 100));
  console.log(utilLoss.value);
  console.log(utilLoss.value / (Math.abs(utilChange.value) / 100));
  if (utilChange.value !== 0 && utilLoss.value!=0) {
    utilTotalAmount.value = (utilLoss.value / (Math.abs(utilChange.value) / 100)).toFixed(2);
    console.log(utilTotalAmount.value)
  }
}

// 计算买入手数
function calculateTotalShare() {
  let nowPrice = store.stockinfoG?.live_data?.price;
  if (nowPrice === undefined || nowPrice === null) {
    console.error("当前价格未定义或为空，无法计算买入手数");
    return;
  }
  if (utilTotalAmount.value !== 0) {
    utilTotalShare.value = ((utilTotalAmount.value / (nowPrice * 100))*100).toFixed(0);
  }
}

</script>

<template>
  <div class="detail column">
    <el-dialog v-model="utilShow" title="计算工具" width="600" draggable
               :modal="false"
               :append-to-body="true"
               :lock-scroll="false"
               :close-on-click-modal="false"
               modal-class="util-modal-wrap"
               class="util-dialog">
      <div class="column">
        <div class="row" style="padding: 5px">
          <div class="row">
            现价:
            <el-input-number v-model="utilPrice1" :controls="false" @change="updateUtil" style="width: 80px"></el-input-number>
          </div>
          <div class="row">
            目标价:
            <el-input-number v-model="utilPrice2" :controls="false" @change="updateUtil" style="width: 80px"></el-input-number>
          </div>
          <div class="row">
            止损额度:
            <el-input-number v-model="utilLoss" :controls="false" @change="updateUtil" style="width: 80px"></el-input-number>
          </div>
        </div>
        <div class="row" style="padding: 20px">
          <div class="row">
            涨跌幅：
            <el-text :style="utilChange > 0 ? { color: 'red' } : utilChange < 0 ? { color: 'green' } : {}">{{ utilChange }}%</el-text>
          </div>
          <div class="row">
            买入总金额：
            <el-text>{{utilTotalAmount}}</el-text>
          </div>
          <div class="row">
            买入份数：
            <el-text>{{utilTotalShare}}</el-text>
          </div>
        </div>
      </div>
    </el-dialog>
    <el-drawer v-model="IntradayChartShow" direction="ltr" size="100%" :modal="false" modal-class="mask-layer" custom-class="custom-drawer">
      <template #default>
<!--        <div @wheel.stop.prevent>-->
<!--          <img :src=ImageSrc  alt="获取分时图失败">-->
<!--          <RTable ></RTable>-->
<!--        </div>-->
        <div @wheel.stop> <!-- 防止滚动穿透，影响下面的蜡烛图也跟着滚动 -->
          <div class="scrollable-content">
            <img :src="ImageSrc" alt="获取分时图失败" style="height: 500px;width:650px;margin-left: 50px">
            <RTable ref="rTableRef" :code="store.stockinfoG!.code" ></RTable>
          </div>
        </div>
      </template>
    </el-drawer>
    <div class="row" style="gap:10px">
      <label style="font-family: 'Arial',serif; display: flex;  ">{{store.stockinfoG!.code}}</label>
      <label :style="{ color: store.stockinfoG!.hold ?'orange':'black' }" style="font-family: 'Adobe 黑体 Std R';font-weight: bold;font-size: 25px">{{store.stockinfoG!.name}}</label>
    </div>
    <el-divider style="margin: 5px"/>
    <div class="row" >
      <label >今开 {{store.stockinfoG?.live_data?.open}}</label>
      <label >最高 {{store.stockinfoG?.live_data?.high}}</label>
      <label >最低 {{store.stockinfoG?.live_data?.low}}</label>
    </div>
    <el-divider />
    <div class="row" style="gap:10px">
      <label >现价 </label>
      <label :style="{color:getColor(store.stockinfoG!.live_data?.percent),fontSize:'18px',fontWeight:'bold'}">{{store.stockinfoG?.live_data?.price}}</label>
      <label >涨跌幅 </label>
      <label :style="{color:getColor(store.stockinfoG!.live_data?.percent),fontSize:'18px',fontWeight:'bold'}">{{store.stockinfoG?.live_data?.percent}}%</label>
    </div>
    <el-divider />
    <div class="ma-container">
      <label>MA5</label>
      <label>{{store.stockinfoG?.live_data?.ma5}}</label>
      <label>MA10</label>
      <label>{{store.stockinfoG?.live_data?.ma10}}</label>
      <label>MA20</label>
      <label>{{store.stockinfoG?.live_data?.ma20}}</label>
      <label>MA60</label>
      <label>{{store.stockinfoG?.live_data?.ma60}}</label>
    </div>
    <el-divider />
    <div class="row" style="gap: 10px">
      <el-tag :class="store.stockinfoG?.rowData?.ma[1]">{{store.stockinfoG?.rowData?.ma[0]}} </el-tag>
      <el-tag :class="store.stockinfoG?.rowData?.box[1]">{{store.stockinfoG?.rowData?.box[0]}} </el-tag>
    </div>
    <el-divider />
    <el-tag :class="store.stockinfoG?.rowData?.advise[1]" style="align-items: center;font-size: 18px">{{store.stockinfoG?.rowData?.advise[0]}} </el-tag>
    <el-divider style="margin: 5px"/>
    <el-button plain @click="IntradayChartShow = true">打开分时图</el-button>
    <el-button plain @click="IntradayChartShow = true">打开历史交易表</el-button>
    <el-button plain @click="utilShow = true">打开工具</el-button>
    <el-button plain @click="manageGroup">修改分组</el-button>
    <StockGroupMange :name="store.stockinfoG!.name" :code="store.stockinfoG!.code" :show-dialog="showGroupManage"></StockGroupMange>
  </div>
</template>
<style>
.detail .el-drawer__body{
  padding:0;
}
/* https://blog.csdn.net/Curry_On/article/details/139442443 */
/* 设置dialog弹出时，其他元素仍能接受焦点*/
.util-dialog{/*dialog仍可以接收焦点*/
  pointer-events: auto;
}
.util-modal-wrap{/*遮罩不要独揽焦点*/
  pointer-events: none;
}
</style>
<style scoped>
.detail{
  width: 100%;
}
.ma-container{
  display: grid;
  grid-template-columns: repeat(4, 1fr); /* 创建四列，每列等宽 */
  gap: 10px; /* 设置列和行之间的间隔 */
}
.text-left {
  text-align: left; /* 左边对齐 */
}

.text-right {
  text-align: right; /* 右边对齐 */
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
.normal{
  color: #000;
}
.custom-drawer {
  position: absolute !important; /* 确保抽屉定位绝对，避免影响布局 */
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  background-color: #0f0f0f; /* 设置背景色 */
}
:deep(.mask-layer){
  /*width: 840px !important;*/
  width: 870px !important;
}
</style>