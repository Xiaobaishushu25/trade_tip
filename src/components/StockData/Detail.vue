<script setup lang="ts">
import {store} from "../../store.ts";

function getColor(percent:number){
  if (percent > 0) {
    return 'red'; // 红色
  } else if (percent < 0) {
    return '#00CD00'; // 绿色
  } else {
    return 'black'; // 黑色
  }
}
function getMaClass(ma:string){
  switch (ma) {
    case '均线多头':
      return 'up-tag';
    case '均线空头':
      return 'down-tag';
    default:
      return 'normal';
  }
}
function getBoxClass(box: string) {
  switch (box) {
    case '已跌破箱体':
    case '上轨区':
      return 'down-tag';
    case '已突破箱体':
    case '下轨区':
      return 'up-tag';
    default:
      return 'normal';
  }
}
// const stockInfoG = store.stockinfoG
</script>

<template>
  <div class="detail column">
    <div class="row" style="gap:10px">
      <label style="font-family: 'Arial'; display: flex;  ">{{store.stockinfoG!.code}}</label>
      <label :style="{ color: store.stockinfoG!.hold ?'orange':'black' }" style="font-family: 'Adobe 黑体 Std R';font-weight: bold;font-size: 25px">{{store.stockinfoG!.name}}</label>
    </div>
    <el-divider />
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
      <el-tag :class="getMaClass(store.rowData.get(store.stockinfoG!.code)?.ma)">{{store.rowData.get(store.stockinfoG!.code)?.ma}} </el-tag>
      <el-tag :class="getBoxClass(store.rowData.get(store.stockinfoG!.code)?.box)">{{store.rowData.get(store.stockinfoG!.code)?.box}} </el-tag>
    </div>
    <el-divider />
    <el-tag :class="store.rowData.get(store.stockinfoG!.code)?.advise[1]" style="align-items: center;font-size: 18px">{{store.rowData.get(store.stockinfoG!.code)?.advise[0]}} </el-tag>

<!--    <label>{{store.stockinfoG}}</label>-->
  </div>

</template>

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
</style>