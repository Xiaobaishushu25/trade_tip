<script setup lang="ts">
import {store} from "../../store.ts";
import {onMounted, ref, watch} from "vue";
import {invoke} from "@tauri-apps/api/core";


const ImageSrc = ref('');
const priceImageShow = ref(false);

// onMounted(async () => {
//   await getStockPriceImg(store.stockinfoG!.code);
// })
watch(() => store.stockinfoG!.code, async (newVal) => {
  // console.log('分组内的的股票changed:', newVal);
  if (priceImageShow.value){
    await getStockPriceImg(newVal);
  }
});
watch(priceImageShow, async (newVal) => {
  if (newVal){
    await getStockPriceImg(store.stockinfoG!.code);
  }
})
function getColor(percent:number){
  if (percent > 0) {
    return 'red'; // 红色
  } else if (percent < 0) {
    return '#00CD00'; // 绿色
  } else {
    return 'black'; // 黑色
  }
}
async function getStockPriceImg(code:string){
  invoke<ArrayBuffer>("query_live_stocks_data_img", {code: code}).then(data => {
    const blob = new Blob([data], { type: 'image/png' }); // 根据你的图片类型设置MIME类型，如'image/jpeg'、'image/png'等
    ImageSrc.value = URL.createObjectURL(blob);
  }).catch(err => {
    console.log(err)
  })
}

</script>

<template>
  <div class="detail column">
    <el-drawer v-model="priceImageShow" direction="ltr" size="100%" :modal="false" modal-class="mask-layer" custom-class="custom-drawer">
      <template #default>
        <div>
          <img  :src=ImageSrc  alt="test">
        </div>
      </template>
    </el-drawer>
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
      <el-tag :class="store.stockinfoG?.rowData?.ma[1]">{{store.stockinfoG?.rowData?.ma[0]}} </el-tag>
      <el-tag :class="store.stockinfoG?.rowData?.box[1]">{{store.stockinfoG?.rowData?.box[0]}} </el-tag>
    </div>
    <el-divider />
    <el-tag :class="store.stockinfoG?.rowData?.advise[1]" style="align-items: center;font-size: 18px">{{store.stockinfoG?.rowData?.advise[0]}} </el-tag>
    <el-divider />
    <el-button type="info" @click="priceImageShow = true">打开分时图</el-button>
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
.custom-drawer {
  position: absolute !important; /* 确保抽屉定位绝对，避免影响布局 */
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
}
::v-deep .mask-layer{
  width: 740px !important;
}
</style>