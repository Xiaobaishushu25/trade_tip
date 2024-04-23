<script lang="ts" setup>
import {useRoute, useRouter} from "vue-router";
import {invoke} from "@tauri-apps/api/core";
import {onMounted, ref, watch} from "vue";
import * as echarts from "echarts/core";
import {StockData} from "../type.ts";
import {WebviewWindow} from "@tauri-apps/api/webviewWindow";
import {EChartsType} from "echarts";

const router = useRouter()
const route = useRoute()
const code = route.params.code
// const props = defineProps({
//   code: {
//     type: String,
//     required: true
//   }
// })

const upColor = 'rgba(255,255,255,0.6)';
const bColor = '#ec0000';
const downColor = 'rgb(55,150,55)';

const chart=ref(null)
// let myChart: any;
let myChart: EChartsType;
// const data = ref<StockData[]>([])
const rawData = ref([])
const tipConfig = ref({
  isVisible:false,
  x:0,
  y:0
})
// watch(() => props.code, (_) => {
//
// });
onMounted(async ()=>{
  myChart = echarts.init(chart.value);
  // myChart = echarts.init(chart.value);
  await query_stocks_day_k_limit();
  myChart.setOption(init_option());
  myChart.on('dataZoom', updatePosition);
})
async function query_stocks_day_k_limit(){
  try {
    const res = await invoke<StockData[]>('query_stocks_day_k_limit', { code: code }); // 使用 await 等待 invoke 完成
    console.log("查到了数据");
    rawData.value = res.reverse(); // 处理查询到的数据
  } catch (err) {
    console.log(err);
  }
  // invoke<StockData[]>('query_stocks_day_k_limit',{code:code}).then((res)=>{
  //   // console.log(res)
  //   // data.value = handleRawData(res)
  //   console.log("查到了数据")
  //   rawData.value = res
  // }).catch((err)=>{
  //   console.log(err)
  // })
}
function handleRawData(raw: StockData[]){
  let categoryData = [];
  let values = [];
  let volumes = [];
  let ma5 = [];
  let ma10 = [];
  let ma20 = [];
  let ma60 = [];
  for (let i = 0; i < raw.length; i++) {
    let element = raw[i];
    categoryData.push(element.date);
    values.push([element.open,element.close,element.low,element.high,element.vol]);
    volumes.push([i, element.vol, element.open > element.close ? 1 : -1]);
    ma5.push(element.ma5);
    ma10.push(element.ma10);
    ma20.push(element.ma20);
    ma60.push(element.ma60);
  }
  return {
    categoryData: categoryData,
    values: values,
    volumes: volumes,
    ma5: ma5,
    ma10: ma10,
    ma20:ma20,
    ma60:ma60
  };
}
function init_option(){
  const data = handleRawData(rawData.value);
  console.log("处理后的数据",data)
  return {
    animation: false,
    legend: {
      bottom: 10,
      left: 'center',
      data: ['酒ETF index', 'MA5', 'MA10', 'MA20', 'MA60']
      // data: ['酒ETF index', ]
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
        if (params.seriesType == "candlestick") {
          htmlContent = `<div class="column tip">
      <label style="background-color: rgb(241, 241, 148)">${params.name}/${computeWeek(params.name)}</label>
      <hr style="margin:0;"> <!-- 添加水平线并设置上下边距 -->
      <label>开盘：${params.data[1]}</label>
      <label>最高：${params.data[4]}</label>
      <label>最低：${params.data[3]}</label>
      <label>收盘：${params.data[2]}</label>
      <label>涨跌：${(params.data[1] - params.data[2]).toFixed(3)}</label>
      <label>涨幅：${calculateChangeRate(params.data[1], params.data[2])}%</label>
      <label>成交量：${params.data[5]}</label>
    </div>`
        } else if (params.seriesType == "line") {
          if (params.seriesName == "MA5") {
            htmlContent = `<label>MA(MA5):${params.value}</label>`
          } else if (params.seriesName == "MA10") {
            htmlContent = `<label>MA(MA10):${params.value}</label>`
          } else if (params.seriesName == "MA20") {
            htmlContent = `<label>MA(MA20):${params.value}</label>`
          } else if (params.seriesName == "MA30") {
            htmlContent = `<label>MA(MA30):${params.value}</label>`
          } else if (params.seriesName == "MA60") {
            htmlContent = `<label>MA(MA60):${params.value}</label>`
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
      show: false,
      feature: {
        dataZoom: {
          yAxisIndex: false
        },
        brush: {
          type: ['lineX', 'clear']
        }
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
        axisLine: {onZero: false},
        splitLine: {show: false},
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
        axisLine: {onZero: false},
        axisTick: {show: false},
        splitLine: {show: false},
        axisLabel: {show: true},
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
        axisLabel: {show: false},
        axisLine: {show: false},
        axisTick: {show: false},
        splitLine: {show: false}
      }
    ],
    dataZoom: [
      {
        type: 'inside',
        xAxisIndex: [0, 1],
        start: 85,
        end: 100,
        zoomOnMouseWheel: "ctrl"// 启用鼠标滚轮触发缩放
      },
      {
        show: true,
        xAxisIndex: [0, 1],
        type: 'slider',
        top: '85%',
        start: 85,
        end: 100
      }
    ],
    series: [
      {
        name: '酒ETF index',
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
        data: data.ma5,
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
        data: data.ma10,
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
        data: data.ma20,
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
        data: data.ma60,
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
        data: data.vol
      },
    ],
  }
}
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
  <div style="border: #535bf2 1px solid;">
  <el-button @click="back()">退回</el-button>
  <div v-shortkey.once="['f10']"  @shortkey="handleKeydown()">
    <!--  <div >-->
    <!-- 为 ECharts 准备一个定义了宽高的 DOM -->
    <div id="main" ref="chart" style="width: 1600px;height:1000px;border: #9d6a09 1px solid "></div>
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