<script setup lang="ts">
import {listen} from "@tauri-apps/api/event";
import {onBeforeMount,onMounted,ref,watch} from "vue";
import * as echarts from "echarts/core";
import {invoke} from "@tauri-apps/api/core";
import {Position} from "../../type.ts";
import {errorNotification} from "../../utils.ts";

const positionChart = ref(null)
let myChart=null;
// const chartData = ref([])
const data = ref<Position[]>([])
watch(data, ()=>{
  if(myChart==null){
    return
  }
  myChart.setOption({
    xAxis: {
      data: data.value.map(item => item.date)
    },
    series: [
      {
        name: '仓位',
        data: data.value.map(item => item.position),
      },
      {
        name: '上证指数',
        data: data.value.map(item => item.sh),
      },
      {
        name: '深证成指',
        data: data.value.map(item => item.sz),
      },
      {
        name: '创业板指',
        data: data.value.map(item => item.cyb),
      },
      {
        name: '上证50',
        data: data.value.map(item => item.sz50),
      },
      {
        name: '沪深300',
        data: data.value.map(item => item.hs300),
      },
      {
        name: '中证500',
        data: data.value.map(item => item.zz500),
      }
    ]
  })
}, {deep: true})

onBeforeMount(async () => {
  invoke<Position[]>('query_all_positions',{}).then(_data =>{
    console.log(_data)
    data.value = _data
  }).catch(e => {errorNotification(`查询历史持仓数据出错${e}`)})
})
// 使用ECharts来初始化图表
onMounted(async () => {
  let unlistenPositionUpdate = await listen("position_update", ({payload}) => {
    // console.log(payload)
    // 这个是修改的，payload是一个数组，第一个元素是日期，第二个元素是仓位
    let olddata = data.value.find(item => item.date==payload[0])
    olddata.position = payload[1]
  })
  let unlistenPositionInsert = await listen("position_insert", ({payload}) => {
    //这个是新增的，payload是一个position对象
    // console.log(payload)
    data.value.push(payload)
  })
  myChart = echarts.init(positionChart.value);
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
        min: (value) => Math.floor(value.min * 0.5), // 向下取整，留出 10% 的下边距
        max: (value) => Math.ceil(value.max * 1.1)   // 向上取整，留出 10% 的上边距
        // boundaryGap: ['20%', '20%'],
      },
      {
        type: 'value',
        name: '仓位(%)',
        position: 'right',
        min: 0,
        max: 110
      }
    ],
    series: [
      {
        name: '仓位',
        type: 'bar',
        data: data.value.map(item => item.position),
        // color: '#5470c6',
        itemStyle: {
          color: (params) => {
            const position = params.data;
            if (position < 40) {
              return '#147e14'; // 绿色
            } else if (position > 70) {
              return '#bd4d4d'; // 红色
            } else {
              return '#a87728'; // 橘黄色
            }
          }
        },
        label: {
          show: true,           // 显示标签
          position: 'top',      // 标签位置在柱顶
          formatter: '{c}',      // 显示数值
          fontSize: 12,         // 设置字体大小
          color: '#000'         // 标签颜色（可以根据需求调整）
        },
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