<script setup lang="ts">
import { writeFile } from '@tauri-apps/plugin-fs';
import {onMounted, ref} from 'vue'
import * as echarts from 'echarts/core';
import {EChartsType, zrender} from "echarts";
import {ZRenderType} from "echarts/types/dist/shared";
import { Window } from '@tauri-apps/api/window';
import {WebviewWindow} from "@tauri-apps/api/webviewWindow";
import {invoke} from "@tauri-apps/api/core";
import {useRouter} from "vue-router";

var ROOT_PATH = 'https://echarts.apache.org/examples';

const router = useRouter()
const chart=ref(null)
var myChart;
var dataItem = [[[3111, 16975], [3122, 17540]]] // 这是第一个 dataItem
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
  myChart = echarts.init(chart.value);
  var option;
  await invoke('get_response',{url:'https://echarts.apache.org/examples/data/asset/data/stock-DJI.json'}).then((res)=>{
    //打印当前时间，按照时分秒的时间格式
    console.log(new Date().toLocaleTimeString())
    console.log("获取成功");
  }).catch((err)=>{
    console.log(err)
  })
  const data1 = (await invoke('get_response',{url:'https://echarts.apache.org/examples/data/asset/data/stock-DJI.json'}))
  const data = JSON.parse(data1)
  console.log(data)
  init(data,myChart,option)
  myChart.setOption({
    graphic: echarts.util.map(dataItem, function(item, dataIndex) {
      console.log("进来绘制图形",item)
      console.log("进来绘制图形,第一个点",myChart.convertToPixel({seriesIndex: 0}, item[0]))
      console.log("进来绘制图形,第二个点",myChart.convertToPixel({seriesIndex: 0}, item[1]))
      return {
        id: 'text1',
        type: 'line',
        shape: {
          x1: myChart.convertToPixel({seriesIndex: 0}, item[0])[0],
          y1: myChart.convertToPixel({seriesIndex: 0}, item[0])[1],
          x2: myChart.convertToPixel({seriesIndex: 0}, item[1])[0],
          y2: myChart.convertToPixel({seriesIndex: 0}, item[1])[1],
          percent: 100
        },
        style: {
          stroke: '#000',
          fill: 'rgb(218,36,36)'
        },
        invisible: false,
        draggable: true,
        silent: false,
        z: 100
      };
    })
  });
  myChart.on('dataZoom', param => {
    console.log("数据缩放了",param)
  })
  myChart.on('dataZoom', updatePosition);
  // const zr: ZRenderType = myChart.getZr()
  zr = myChart.getZr()
  zr.on("keydown",event=>{
    console.log(event)
    if (event.keyCode ==113){
      tipConfig.value.isVisible = true
      tipConfig.value.x = event.offsetX
      tipConfig.value.y = event.offsetY
    }
  })
  myChart.on('selectchanged', param => {
    console.log(param)
  })
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
    let xIndex = myChart.convertFromPixel({ seriesIndex: 0 }, [params.offsetX, params.offsetY])
    console.log("索引是",xIndex)
    let yun = myChart.convertToPixel({ seriesIndex: 0 }, xIndex)
    console.log("转换回来是",yun)
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
  })
})
function updatePosition() {
  console.log("更新点了")
  myChart.setOption({
    graphic: dataItem.map(function (item, dataIndex) {
      return {
        id: 'text1',
        shape: {
          x1: myChart.convertToPixel({seriesIndex: 0}, item[0])[0],
          y1: myChart.convertToPixel({seriesIndex: 0}, item[0])[1],
          x2: myChart.convertToPixel({seriesIndex: 0}, item[1])[0],
          y2: myChart.convertToPixel({seriesIndex: 0}, item[1])[1],
          percent: 100
        },
      };
    })
  });
}
function init(rawData:number[][],myChart:EChartsType,option) {
  const data = splitData(rawData);
  console.log("切分后的数据是",data)
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
            let htmlContent;
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
            return htmlContent
          }
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
          show:false,
          feature: {
            dataZoom: {
              yAxisIndex: false
            },
            brush: {
              type: ['lineX', 'clear']
            }
          }
        },
        // brush: {
        //   xAxisIndex: 'all',
        //   brushLink: 'all',
        //   outOfBrush: {
        //     colorAlpha: 0.1
        //   }
        // },
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
          },
          {
            name:"custom",
            type: 'custom',
            renderItem: function (params, api) {
              // 对于 data 中的每个 dataItem，都会调用这个 renderItem 函数。
              // （但是注意，并不一定是按照 data 的顺序调用）
              console.log("api是",api)
              // 这里进行一些处理，例如，坐标转换。
              // 这里使用 api.value(0) 取出当前 dataItem 中第一个维度的数值。
              var categoryIndex = api.value(0);
              // 这里使用 api.coord(...) 将数值在当前坐标系中转换成为屏幕上的点的像素值。
              var startPoint = api.coord([api.value(1), categoryIndex]);
              var endPoint = api.coord([api.value(2), categoryIndex]);
              // 这里使用 api.size(...) 获得 Y 轴上数值范围为 1 的一段所对应的像素长度。
              var height = api.size([0, 10000])[1] * 0.6;

              // shape 属性描述了这个矩形的像素位置和大小。
              // 其中特殊得用到了 echarts.graphic.clipRectByRect，意思是，
              // 如果矩形超出了当前坐标系的包围盒，则剪裁这个矩形。
              // 如果矩形完全被剪掉，会返回 undefined.
              //打印这个矩形的xy和长宽
              var rectShape = echarts.graphic.clipRectByRect({
                // 矩形的位置和大小。
                x: startPoint[0],
                y: startPoint[1] - height / 2,
                width: endPoint[0] - startPoint[0],
                height: height
              }, {
                // 当前坐标系的包围盒。
                x: params.coordSys.x,
                y: params.coordSys.y,
                width: params.coordSys.width,
                height: params.coordSys.height
              });
              // 这里返回为这个 dataItem 构建的图形元素定义。
              return rectShape && {
                // 表示这个图形元素是矩形。还可以是 'circle', 'sector', 'polygon' 等等。
                type: 'rect',
                shape: rectShape,
                // 用 api.style(...) 得到默认的样式设置。这个样式设置包含了
                // option 中 itemStyle 的配置和视觉映射得到的颜色。
                style: api.style()
              };
            },
            data: [
              [400, 100, 500, 200], // 这是第一个 dataItem
              [53, 31, 21, 56], // 这是第二个 dataItem
              [71, 33, 10, 20], // 这是第三个 dataItem
            ]
          },
          // {
          //   name:"custom1",
          //   type: 'custom',
          //   renderItem: function (params, api) {
          //     // 对于 data 中的每个 dataItem，都会调用这个 renderItem 函数。
          //     // （但是注意，并不一定是按照 data 的顺序调用）
          //     // 这里进行一些处理，例如，坐标转换。
          //     // 这里使用 api.coord(...) 将数值在当前坐标系中转换成为屏幕上的点的像素值。
          //     var startPoint = api.coord([api.value(0), api.value(1)]);
          //     var endPoint = api.coord([api.value(2), api.value(3)]);
          //     // 这里使用 api.size(...) 获得 Y 轴上数值范围为 1 的一段所对应的像素长度。
          //     let line = new echarts.graphic.Line({
          //       shape:{
          //         x1: startPoint[0],
          //         y1: startPoint[1],
          //         x2: endPoint[0],
          //         y2: endPoint[1],
          //         percent: 100
          //       },
          //       style:{
          //         fill:'red',
          //       }
          //     });
          //     line.draggable = true;
          //     return line
          //   },
          //   data: [
          //     [3096, 16942, 3100, 17525], // 这是第一个 dataItem
          //   ],
          //   draggable: true,
          // }
        ],
      }),
      true
  );
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
function calculateChangeRate(openPrice:number, closePrice:number) {
  // 计算涨跌幅
  const changeRate = ((closePrice - openPrice) / openPrice) * 100;
  // 保留两位小数并返回
  return changeRate.toFixed(2);
}
async function handleKeydown(){
  const appWindow = WebviewWindow.getByLabel('tool')
  console.log(appWindow);
  console.log(appWindow?.window);
  // console.log(appWindow?.label);
  console.log(await appWindow?.isVisible())
  await appWindow?.show()
}
function back(){
  router.back();
}
</script>

<template>
  <div v-shortkey.once="['f10']"  @shortkey="handleKeydown()">
<!--  <div >-->
    <!-- 为 ECharts 准备一个定义了宽高的 DOM -->
    <div id="main" ref="chart" style="width: 1500px;height:1000px;"></div>
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
  </div>
  <el-button @click="back()">退回</el-button>
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