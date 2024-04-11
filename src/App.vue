<script setup lang="ts">
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
import { writeFile } from '@tauri-apps/plugin-fs';
import {onMounted, ref} from 'vue'
import * as echarts from 'echarts/core';
import axios from "axios";
import {EChartsType, zrender} from "echarts";
import {ZRenderType} from "echarts/types/dist/shared";
var ROOT_PATH = 'https://echarts.apache.org/examples';

// var chartDom = document.getElementById('main');
// var myChart = echarts.init(chartDom);

const chart=ref(null)
const tip=ref(null)
const tipConfig = ref({
  isVisible:false,
  x:0,
  y:0
})
const state = ref({
  isfirst:true,
  x:0,
  y:0
})
// const upColor = '#ec0000';
const upColor = 'rgba(255,255,255,0.6)';
const bColor = '#ec0000';
const downColor = 'rgb(55,150,55)';
var zr: ZRenderType
var lline: echarts.graphic.Line
onMounted(async ()=>{
  var chartDom = document.getElementById('main');
  var myChart: EChartsType = echarts.init(chartDom);
  // var myChart = echarts.init(chart);
  var option;

  var data = await axios.get(ROOT_PATH + '/data/asset/data/stock-DJI.json');
  init(data.data,myChart,option)
  // const zr: ZRenderType = myChart.getZr()
  zr = myChart.getZr()
  myChart.getZr().on('mousemove', params => {
    if (state.value.isfirst){
    }else{
      zr.remove(lline)
      lline = new echarts.graphic.Line({
        shape:{
          x1: state.value.x,
          y1: state.value.y,
          x2: params.offsetX,
          y2: params.offsetY,
          percent: 100
        },
        style:{
          fill:'red',
        },
      })
      lline.draggable = true
      zr.add(lline)
    }
  })
  myChart.getZr().on('click', params => {
    let pointInPixel = [params.offsetX, params.offsetY]
    if (state.value.isfirst){
      state.value.isfirst = false
      state.value.x = params.offsetX
      state.value.y = params.offsetY
    }else{
      state.value.isfirst = true
      lline = new echarts.graphic.Line({
        shape:{
          x1: state.value.x,
          y1: state.value.y,
          x2: params.offsetX,
          y2: params.offsetY,
          percent: 100
        },
        style:{
          fill:'red',
        },
        // smooth: true,
        onclick(e:MouseEvent) {
            console.log(e)
          zr.remove(lline)
          // e.cancelBubble = true
          e.stopPropagation()
        },
      })
      lline.draggable = true
      zr.add(lline)
    }

    // tipConfig.value = {
    //   isVisible : true,
    //   x:params.offsetX,
    //   y:params.offsetY
    // }
    // tipConfig.value.isVisible = true
    // tipConfig.value.x = params.offsetX
    // tipConfig.value.y = params.offsetY
    // var line = new echarts.graphic.Line({
    //   shape:{
    //     x1: 10,
    //     y1: params.offsetY,
    //     x2: 5000,
    //     y2: params.offsetY,
    //     percent: 100
    //   },
    //   style:{
    //     fill:'red',
    //   },
    //   onclick(e:MouseEvent) {
    //       console.log(e)
    //     zr.remove(line)
    //     // e.cancelBubble = true
    //     e.stopPropagation()
    //   },
    // })
    // line.draggable = true
    // let text = new echarts.graphic.Text({
    //   style:{
    //     text:"上轨",
    //     x:params.offsetX,
    //     y:params.offsetY
    //   }
    // });
    // text.draggable = true
    // line.setTextContent(text)
    // zr.add(line)
    // tip?.value?.focus()
    // if (myChart.containPixel('grid', pointInPixel)) {
    //   let xIndex = myChart.convertFromPixel({ seriesIndex: 0 }, [params.offsetX, params.offsetY])[0]
    //   console.log(xIndex)
    // }
    // var circle = new zrender.Circle({
    // var rect = new echarts.graphic.Rect({
    //   position:[Math.random() * 200, Math.random() * 200],
    //   scale: [1,1],
    //   shape:{
    //     x:10,
    //     y:20,
    //     width:100,
    //     height:50
    //   },
    //   style:{
    //     fill:'red',
    //   },
    //   zlevel:3
    // });
    // zr.add(rect);
  })
  // console.dir(`配置是${myChart.getOption()}`)
  // myChart.appendData({seriesIndex:1,data:[
  //     "2004-01-02",
  //     10452.74,
  //     10409.85,
  //     10367.41,
  //     10554.96,
  //     168890000
  //   ],})
})
// option && myChart.setOption(option);
function init(rawData:number[][],myChart:EChartsType,option) {
  var data = splitData(rawData);
  // console.log("数据是"+data)
  myChart.setOption(
      (option = {
        animation: false,
        legend: {
          bottom: 10,
          left: 'center',
          data: ['Dow-Jones index', 'MA5', 'MA10', 'MA20', 'MA60']
        },
        tooltip: {
          // trigger: 'axis',// https://echarts.apache.org/zh/option.html#grid.tooltip.trigger
          trigger: 'item',
          axisPointer: {
            type: 'cross',
            // snap:true 坐标轴指示器是否自动吸附到点上。默认自动判断。好像是默认启用 https://echarts.apache.org/zh/option.html#grid.tooltip.axisPointer.snap
          },
          borderWidth: 1,
          borderColor: '#776d6d',
          padding: 5,
          textStyle: {
            color: '#000'
          },
          formatter: function (params: any) { // params 是 formatter 需要的数据集
            var htmlContent
            if (params.seriesType=="candlestick"){
              htmlContent = `<div class="column tip">
      <label style="background-color: rgb(241, 241, 148)">${params.name}/${computeWeek(params.name)}</label>
      <hr style="margin:0;"> <!-- 添加水平线并设置上下边距 -->
      <label>开盘：${params.data[1]}</label>
      <label>最高：${params.data[4]}</label>
      <label>最低：${params.data[3]}</label>
      <label>收盘：${params.data[2]}</label>
      <label>涨跌：${(params.data[1]-params.data[2]).toFixed(3)}</label>
      <label>涨幅：${calculateChangeRate(params.data[1],params.data[2])}%</label>
      <label>成交量：${params.data[5]}</label>
    </div>`
            }else if(params.seriesType=="line"){
              if (params.seriesName=="MA5"){
                htmlContent =`<label>MA(MA5):${params.value}</label>`
              }else if (params.seriesName=="MA10"){
                htmlContent =`<label>MA(MA10):${params.value}</label>`
              }else if (params.seriesName=="MA20"){
                htmlContent =`<label>MA(MA20):${params.value}</label>`
              }else if (params.seriesName=="MA30"){
                htmlContent =`<label>MA(MA30):${params.value}</label>`
              }else if (params.seriesName=="MA60"){
                htmlContent =`<label>MA(MA60):${params.value}</label>`
              }
            }
            // console.log(params.name)//日期：2016-04-28
            // console.log(params.data)//[3102, 18023.88, 17830.76, 17796.55, 18035.73, 100920000]
            return htmlContent
          }
          // position: function (pos, params, el, elRect, size) {
          //   const obj = {
          //     top: 10
          //   };
          //   obj[['left', 'right'][+(pos[0] < size.viewSize[0] / 2)]] = 30;
          //   return obj;
          // }
          // extraCssText: 'width: 170px'
        },

        axisPointer: {
          link: [
            {
              xAxisIndex: 'all'
            }
          ],
          label: {
            backgroundColor: '#d5abab'
          }
        },
        toolbox: {
          feature: {
            dataZoom: {
              yAxisIndex: false
            },
            brush: {
              type: ['lineX', 'clear']
            }
          }
        },
        brush: {
          xAxisIndex: 'all',
          brushLink: 'all',
          outOfBrush: {
            colorAlpha: 0.1
          }
        },
        visualMap: {
          show: false,
          seriesIndex: 5,
          dimension: 2,
          pieces: [
            {
              value: 1,
              color: downColor
            },
            {
              value: -1,
              // color: upColor
              color: bColor
            }
          ]
        },
        grid: [
          {
            left: '10%',
            right: '8%',
            height: '50%'
          },
          {
            left: '10%',
            right: '8%',
            top: '63%',
            height: '16%'
          }
        ],
        xAxis: [
          {
            type: 'category',
            data: data.categoryData,
            boundaryGap: false,
            axisLine: { onZero: false },
            splitLine: { show: false },
            min: 'dataMin',
            max: 'dataMax',
            axisPointer: {
              z: 100
            }
          },
          {
            type: 'category',
            gridIndex: 1,
            data: data.categoryData,
            boundaryGap: false,
            axisLine: { onZero: false },
            axisTick: { show: false },
            splitLine: { show: false },
            axisLabel: { show: true },
            min: 'dataMin',
            max: 'dataMax'
          }
        ],
        yAxis: [
          {
            scale: true,
            splitArea: {
              show: true
            }
          },
          {
            scale: true,
            gridIndex: 1,
            splitNumber: 2,
            axisLabel: { show: false },
            axisLine: { show: false },
            axisTick: { show: false },
            splitLine: { show: false }
          }
        ],
        dataZoom: [
          {
            type: 'inside',
            xAxisIndex: [0, 1],
            start: 98,
            end: 100,
            zoomOnMouseWheel: "ctrl"// 启用鼠标滚轮触发缩放
          },
          {
            show: true,
            xAxisIndex: [0, 1],
            type: 'slider',
            top: '85%',
            start: 98,
            end: 100
          }
        ],
        series: [
          {
            name: 'Dow-Jones index',
            type: 'candlestick',
            data: data.values,
            itemStyle: {
              color: upColor,
              color0: downColor,
              borderColor: bColor,
              borderColor0: undefined
            },
          },
          {
            name: 'MA5',
            type: 'line',
            data: calculateMA(5, data),
            smooth: true,
            // symbol:'none',
            symbolSize: 0,
            lineStyle: {
              opacity: 0.5
            },
            // tooltip: {
            //   trigger: 'axis',// https://echarts.apache.org/zh/option.html#grid.tooltip.trigger
            //   // trigger: 'item',
            //   axisPointer: {
            //     type: 'cross'
            //   },
            //   borderWidth: 1,
            //   borderColor: '#776d6d',
            //   padding: 5,
            //   textStyle: {
            //     color: '#000'
            //   },
            //   formatter: function (params,ticket: string) { // params 是 formatter 需要的数据集
            //     console.log("均线5",params)
            //   }
            //   }
          },
          {
            name: 'MA10',
            type: 'line',
            data: calculateMA(10, data),
            smooth: true,
            // symbol:'none',
            symbolSize: 0,
            lineStyle: {
              opacity: 0.5
            }
          },
          {
            name: 'MA20',
            type: 'line',
            // symbol:'none',
            symbolSize: 0,
            data: calculateMA(20, data),
            smooth: true,
            lineStyle: {
              opacity: 0.5
            }
          },
          {
            name: 'MA60',
            type: 'line',
            // symbol:'none',
            symbolSize: 0,
            data: calculateMA(60, data),
            smooth: true,
            lineStyle: {
              opacity: 0.5
            },
          },
          {
            name: 'Volume',
            type: 'bar',
            xAxisIndex: 1,
            yAxisIndex: 1,
            data: data.volumes
          }
        ]
      }),
      true
  );
  myChart.dispatchAction({
    type: 'brush',
    areas: [
      {
        brushType: 'lineX',
        coordRange: ['2016-06-02', '2016-06-20'],
        xAxisIndex: 0
      }
    ]
  });
  option && myChart.setOption(option);
}
function splitData(rawData: number[][]) {
  let categoryData = [];
  let values = [];
  let volumes = [];
  for (let i = 0; i < rawData.length; i++) {
    categoryData.push(rawData[i].splice(0, 1)[0]);
    values.push(rawData[i]);
    volumes.push([i, rawData[i][4], rawData[i][0] > rawData[i][1] ? 1 : -1]);
  }
  return {
    categoryData: categoryData,
    values: values,
    volumes: volumes
  };
}
function calculateMA(dayCount: number, data: { values: number[][] }) {
  var result = [];
  for (var i = 0, len = data.values.length; i < len; i++) {
    if (i < dayCount) {
      result.push('-');
      continue;
    }
    var sum = 0;
    for (var j = 0; j < dayCount; j++) {
      sum += data.values[i - j][1];
    }
    result.push(+(sum / dayCount).toFixed(3));
  }
  return result;
}
async function down(){
  let encoder = new TextEncoder();
  let data = encoder.encode("Hello World");
  await writeFile('file.txt', data, );
}
function addTip(){
  let text = new echarts.graphic.Text({
    style:{
      text:tip.value.value,
      x:tipConfig.value.x,
      y:tipConfig.value.y,
    }
  });
  text.draggable = true
  zr.add(text)
  tipConfig.value.isVisible = false
  tip.value.value=""
}
function focusChange(){
  console.log("焦点改变了")
}
function computeWeek(date:string){
  // console.log(date)
  var date = new Date(date)
  var day = date.getDay();
  // 根据获取的数字，转换为对应的中文星期几
  var weekDays = ["日", "一", "二", "三", "四", "五", "六"];
  var weekDay = weekDays[day];

  // console.log(weekDay); // 输出：周四
  return weekDay
}
function calculateChangeRate(openPrice, closePrice) {
  // 计算涨跌幅
  const changeRate = ((closePrice - openPrice) / openPrice) * 100;

  // 保留两位小数并返回
  return changeRate.toFixed(2);
}
</script>

<template>
  <div class="container">
    <button @click="down()">下载</button>
    <!-- 为 ECharts 准备一个定义了宽高的 DOM -->
    <div id="main"  ref="chart" style="width: 1500px;height:1000px;"></div>
<!--    <input ref="tip" style="width: 60px;font-size: 12px;visibility: hidden">-->
    <input v-if="tipConfig.isVisible" ref="tip" :style="{
      width: `60px`,
      fontSize:`11px`,
        position: 'absolute',
        // position: 'relative',
        left: `${tipConfig.x}px`,
        top: `${tipConfig.y}px`,
        // visibility: tipConfig.isVisible ? 'visible' : 'hidden'
      }"
           @keydown.enter = "addTip()"
           @focus = "focusChange()"
    >
    <div>
      <button @click="">文字</button>
    </div>
    <div class="column tip">
      <label style="background-color: rgb(241, 241, 148)">2022-15-2/五</label>
      <hr style="margin:0;"> <!-- 添加水平线并设置上下边距 -->
      <label>开盘：0.391</label>
      <label>最高：0.391</label>
      <label>最低：0.391</label>
      <label>收盘：0.391</label>
      <label>涨跌：0.391</label>
      <label>涨幅：0.391</label>
      <label>成交量：0.391</label>
    </div>
  </div>
</template>

<style scoped>
.tip{
  font-family:"Adobe 黑体 Std R",serif;
  font-size: 13px;
  border-color: #24c8db;
  border-width: 1px;
  border-style: solid; /* 添加边框样式 */
  width: 110px;
  background-color: #ecece4;
}
</style>
