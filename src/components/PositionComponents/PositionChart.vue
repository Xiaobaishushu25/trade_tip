<script setup lang="ts">
import {listen} from "@tauri-apps/api/event";
import {onBeforeMount,onMounted,ref} from "vue";
import * as echarts from "echarts/core";
import {invoke} from "@tauri-apps/api/core";
import {Position} from "../../type.ts";
import {errorNotification} from "../../utils.ts";

const positionChart = ref(null)
// const chartData = ref([])
const data = ref<Position[]>([])
// const data = ref([
//   {
//     date: '2024-11-01',
//     position: 50,
//     sh: 3500,
//     sz: 14000,
//     cyb: 2500,
//     sz50: 3000,
//     hs300: 4000,
//     zz500: 5000
//   },
//   {
//     date: '2024-11-02',
//     position: 55,
//     sh: 3550,
//     sz: 14200,
//     cyb: 2550,
//     sz50: 3050,
//     hs300: 4050,
//     zz500: 5050
//   },
//   // 继续添加数据...
// ]);
onBeforeMount(async () => {
  invoke<Position[]>('query_all_positions',{}).then(_data =>{
    console.log(_data)
    data.value = _data
  }).catch(e => {errorNotification(`查询历史持仓数据出错${e}`)})
})
// 使用ECharts来初始化图表
onMounted(async () => {
  let unlistenPositionUpdate = await listen("position_change", ({payload }) => {
    console.log(payload)
  })
  const myChart = echarts.init(positionChart.value);
  const option = {
    tooltip: {
      trigger: 'axis'
    },
    grid: {
      left: '10%',   // 左边距
      right: '10%',  // 右边距
      top: '10%',    // 上边距
      bottom: '5%'   // 调整下边距以减少空白
    },
    legend: {
      data: ['仓位', '上证指数', '深证成指', '创业板指', '上证50', '沪深300', '中证500'],
      selected: {
        '仓位': true,  // 默认显示 Position
        '上证指数': true,        // 默认显示 SH
        '深证成指': false,
        '创业板指': false,
        '上证50': false,
        '沪深300': false,
        '中证500': false
      },
    },
    xAxis: {
      type: 'category',
      data: data.value.map(item => item.date)
    },
    yAxis: [
      {
        type: 'value',
        name: '指标值',
        position: 'left',
        boundaryGap: ['20%', '20%'],
      },
      {
        type: 'value',
        name: '仓位(%)',
        position: 'right',
        min: 0,
        max: 100
      }
    ],
    series: [
      {
        name: '仓位',
        type: 'bar',
        data: data.value.map(item => item.position),
        color: '#5470c6',
        yAxisIndex: 1 // 使用右侧坐标轴
      },
      {
        name: '上证指数',
        type: 'line',
        data: data.value.map(item => item.sh),
        smooth: true,
        yAxisIndex: 0 // 使用左侧坐标轴
      },
      {
        name: '深证成指',
        type: 'line',
        data: data.value.map(item => item.sz),
        smooth: true,
        yAxisIndex: 0
      },
      {
        name: '创业板指',
        type: 'line',
        data: data.value.map(item => item.cyb),
        smooth: true,
        yAxisIndex: 0
      },
      {
        name: '上证50',
        type: 'line',
        data: data.value.map(item => item.sz50),
        smooth: true,
        yAxisIndex: 0
      },
      {
        name: '沪深300',
        type: 'line',
        data: data.value.map(item => item.hs300),
        smooth: true,
        yAxisIndex: 0
      },
      {
        name: '中证500',
        type: 'line',
        data: data.value.map(item => item.zz500),
        smooth: true,
        yAxisIndex: 0
      }
    ]
  };
  // 使用配置项显示图表
  myChart.setOption(option);
});
</script>

<template>
  <div id="chart" ref="positionChart" style="width: 100%; height: 520px; padding-top: 15px;"></div>
</template>

<style scoped>

</style>