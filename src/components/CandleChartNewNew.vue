<!--<script lang="ts" setup>-->
<!--import {invoke} from "@tauri-apps/api/core";-->
<!--import {onMounted, ref, watch} from "vue";-->
<!--import * as echarts from "echarts/core";-->
<!--import {Graphic, Line, PaintState, StockData} from "../type.ts";-->
<!--import {WebviewWindow} from "@tauri-apps/api/webviewWindow";-->
<!--import {EChartsType} from "echarts";-->
<!--import {store} from "../store.ts";-->
<!--import {debounce, generateId} from "../utils.ts"-->
<!--import {listen} from "@tauri-apps/api/event";-->

<!--let isCtrlPressed = false;-->
<!--// const route = useRoute()-->
<!--// let code = route.params.code-->
<!--// // watch(() => route.params.code, (_) => {-->
<!--// //   console.log("code变换了",code)-->
<!--// //   query_stocks_day_k_limit();-->
<!--// // });-->
<!--// watch(-->
<!--//     () => router.currentRoute.value,-->
<!--//     (newValue: any) => {-->
<!--//       console.log('newValue',newValue.params.code)-->
<!--//       code = newValue.params.code;-->
<!--//       query_stocks_day_k_limit();-->
<!--//     },-->
<!--//     { immediate: true }-->
<!--// )-->
<!--// const props = defineProps({-->
<!--//   code: {-->
<!--//     type: String,-->
<!--//     required: true-->
<!--//   }-->
<!--// })-->
<!--let code = store.stockinfoG!.code-->
<!--watch(() => store.stockinfoG, (newValue: any) => {-->
<!--  if(code!=newValue.code){-->
<!--    code = newValue.code;-->
<!--    console.log("info code变化了",code)-->
<!--    query_stocks_day_k_limit();-->
<!--    query_graphic();-->
<!--  }-->
<!--},{deep:true})-->
<!--// watch(() => store.count, (newValue: any) => {-->
<!--//   code = newValue;-->
<!--//   console.log("count code变化了",code)-->
<!--//   query_stocks_day_k_limit();-->
<!--// })-->
<!--// watch('stockinfoG', (newValue: StockInfoG | undefined) => {-->
<!--//   if (newValue) {-->
<!--//     code = newValue.code;-->
<!--//     console.log("code变化了");-->
<!--//     query_stocks_day_k_limit();-->
<!--//   }-->
<!--// });-->

<!--const upColor = 'rgba(255,255,255,0.6)';-->
<!--const bColor = '#ec0000';-->
<!--const downColor = 'rgb(55,150,55)';-->

<!--const chart=ref(null)-->
<!--// let myChart: any;-->
<!--let myChart: EChartsType;-->
<!--// const data = ref<StockData[]>([])-->
<!--const rawData = ref([])-->
<!--const lineData = ref<Line[]>([])-->
<!--const newLineData = ref<Graphic[]>([])-->
<!--const tipConfig = ref({-->
<!--  isVisible:false,-->
<!--  x:0,-->
<!--  y:0-->
<!--})-->

<!--watch(lineData, (_) => {-->
<!--  console.log("lineData变化了",lineData.value)-->
<!--  updateLineOption()-->
<!--},{deep:true});-->
<!--// watch(() => props.code, (_) => {-->
<!--//-->
<!--// });-->
<!--onMounted(async ()=>{-->

<!--  myChart = echarts.init(chart.value);-->
<!--  // myChart = echarts.init(chart.value);-->
<!--  await query_stocks_day_k_limit();-->
<!--  await query_graphic();-->
<!--  // myChart.setOption(init_option());-->
<!--  myChart.on('dataZoom', updateLineOption);-->
<!--  await listen('paint', (event) => {-->
<!--    // event.event is the event name (useful if you want to use a single callback fn for multiple event types)-->
<!--    // event.payload is the payload object-->
<!--    handlePaint(event.payload.state);-->
<!--  })-->
<!--  // 监听鼠标滚轮事件-->
<!--  const debouncedFunction = debounce(scrollEvent, 700, true);-->
<!--  document.addEventListener('wheel', (event: WheelEvent) => {-->
<!--    // 这里可以获取滚动事件的信息-->
<!--    const deltaY = event.deltaY; // 滚动的垂直距离-->
<!--    debouncedFunction(deltaY)-->
<!--    // scrollEvent(deltaY)-->
<!--    // 如果你需要阻止页面滚动或其他默认行为-->
<!--    // event.preventDefault();-->
<!--  });-->
<!--  window.addEventListener('resize', function() {-->
<!--    // 当窗口大小变化时，这个函数会被调用-->
<!--    myChart.resize();-->
<!--  });-->
<!--  document.addEventListener('keydown', (event: KeyboardEvent) => {-->
<!--    if (event.ctrlKey) {-->
<!--      if (!isCtrlPressed){-->
<!--        isCtrlPressed = true;-->
<!--        console.log("解锁图表缩放")-->
<!--        myChart.setOption({-->
<!--          dataZoom: [-->
<!--            {-->
<!--              type: 'inside',-->
<!--              xAxisIndex: [0, 1],-->
<!--              zoomLock: false-->
<!--            },-->
<!--          ],-->
<!--        })-->
<!--      }-->
<!--    }-->
<!--  });-->

<!--  document.addEventListener('keyup', (event: KeyboardEvent) => {-->
<!--    if (event.key === 'Control') {-->
<!--      console.log("锁定图表缩放")-->
<!--      if (isCtrlPressed){-->
<!--        isCtrlPressed = false;-->
<!--        myChart.setOption({-->
<!--          dataZoom: [-->
<!--            {-->
<!--              type: 'inside',-->
<!--              xAxisIndex: [0, 1],-->
<!--              zoomLock: true-->
<!--            },-->
<!--          ],-->
<!--        })-->
<!--      }-->
<!--    }-->
<!--  });-->
<!--})-->
<!--// 创建一个映射来存储每个id对应的group-->
<!--const groupMap = new Map<string, { group: any }>();-->
<!--function handleGroupData(graphics:Graphic[]){-->
<!--  graphics.forEach((item) => {-->
<!--    let start = myChart?.convertToPixel({seriesIndex: 0}, item.start);-->
<!--    let end = myChart?.convertToPixel({seriesIndex: 0}, item.end);-->
<!--    if (!start) {-->
<!--      return; // 如果转换失败，则跳过当前项-->
<!--    }-->
<!--    const groupId = item.graphic_id;-->
<!--    let groupEntry = groupMap.get(groupId);-->
<!--    let graphic;-->
<!--    if (item.graphic_type === 'line') {-->
<!--      graphic = {-->
<!--        type: 'line',-->
<!--        id: item.id, // 使用graphic_id或id作为line的id-->
<!--        shape: {-->
<!--          x1: start[0],-->
<!--          y1: start[1],-->
<!--          x2: end[0],-->
<!--          y2: end[1],-->
<!--        },-->
<!--        style: {-->
<!--          lineWidth: 5,-->
<!--        },-->
<!--      }-->
<!--    }else if (item.graphic_type === 'text') {-->
<!--      graphic = {-->
<!--        type:"text",-->
<!--        id: item.id,-->
<!--        position: [start[0], start[1]],-->
<!--        style: {-->
<!--          // stroke: item.config.color,-->
<!--          // lineWidth: item.line_width,-->
<!--          lineWidth: 1,-->
<!--          text: item.content,-->
<!--          // lineDash: item.line_dash,-->
<!--        },-->
<!--        draggable: true,-->
<!--        cursor: 'move',-->
<!--      }-->
<!--    }-->
<!--    if (!groupEntry) {-->
<!--      // 如果没有找到对应的group，则创建一个新的group和lines数组-->
<!--      groupEntry = {-->
<!--        group: {-->
<!--          type: 'group',-->
<!--          id: item.graphic_id,-->
<!--          draggable: true,-->
<!--          children: [graphic]-->
<!--        }-->
<!--      };-->
<!--      groupMap.set(groupId, groupEntry);-->
<!--    } else {-->
<!--      groupEntry.group.children.push(graphic)-->
<!--    }-->
<!--  })-->
<!--}-->
<!--async function query_graphic(){-->
<!--  try {-->
<!--    const res = await invoke<Graphic[]>('query_graphic_by_code', { code: code }); // 使用 await 等待 invoke 完成-->
<!--    console.log("查到了图形数据",res);-->
<!--    console.log("颜色是",res[0].style)-->
<!--    newLineData.value = res;-->
<!--    handleGroupData(res);-->
<!--    myChart.setOption({-->
<!--      graphic:Array.from(groupMap.values()).map(function (item) {-->
<!--        return item.group;-->
<!--      })-->
<!--    })-->
<!--    console.log("配置是", myChart.getOption().options)-->
<!--    // myChart.setOption({-->
<!--    //   graphic:newLineData.value.map(function (item:Graphic, dataIndex) {-->
<!--    //     if (item.graphic_type==="line"){-->
<!--    //       let start = myChart.convertToPixel({seriesIndex: 0}, item.start);-->
<!--    //       console.log("start",start)-->
<!--    //       let end = myChart.convertToPixel({seriesIndex: 0}, item.end);-->
<!--    //       console.log("end",end)-->
<!--    //       return {-->
<!--    //         type: 'group',-->
<!--    //         id: item.graphic_id,-->
<!--    //         draggable:true,-->
<!--    //         children:[-->
<!--    //           {-->
<!--    //             type:"line",-->
<!--    //             id: item.id,-->
<!--    //             shape: {-->
<!--    //               x1: start[0],-->
<!--    //               y1: start[1],-->
<!--    //               x2: end[0],-->
<!--    //               y2: end[1],-->
<!--    //             },-->
<!--    //             style: {-->
<!--    //               // stroke: item.config.color,-->
<!--    //               // lineWidth: item.line_width,-->
<!--    //               lineWidth: 5,-->
<!--    //               // lineDash: item.line_dash,-->
<!--    //             },-->
<!--    //             draggable: true,-->
<!--    //             cursor: 'move',-->
<!--    //           }-->
<!--    //         ]-->
<!--    //       }-->
<!--    //     }else if (item.graphic_type==="text"){-->
<!--    //       let start = myChart.convertToPixel({seriesIndex: 0}, item.start);-->
<!--    //       console.log("start",start)-->
<!--    //       return {-->
<!--    //         type: 'group',-->
<!--    //         id: item.graphic_id,-->
<!--    //         action: 'replace',-->
<!--    //         draggable:true,-->
<!--    //         children:[-->
<!--    //           {-->
<!--    //             type:"text",-->
<!--    //             id: item.id,-->
<!--    //             position: [start[0], start[1]],-->
<!--    //             style: {-->
<!--    //               // stroke: item.config.color,-->
<!--    //               // lineWidth: item.line_width,-->
<!--    //               lineWidth: 1,-->
<!--    //               text: item.content,-->
<!--    //               // lineDash: item.line_dash,-->
<!--    //             },-->
<!--    //             draggable: true,-->
<!--    //             cursor: 'move',-->
<!--    //           }-->
<!--    //         ]-->
<!--    //       }-->
<!--    //     }-->
<!--    //   })-->
<!--    // })-->
<!--    // myChart.hideLoading();-->
<!--  } catch (err) {-->
<!--    console.log(err);-->
<!--  }-->
<!--}-->
<!--async function query_stocks_day_k_limit(){-->
<!--  try {-->
<!--    const res = await invoke<StockData[]>('query_stocks_day_k_limit', { code: code }); // 使用 await 等待 invoke 完成-->
<!--    rawData.value = res.reverse(); // 处理查询到的数据-->
<!--    myChart.setOption(init_option())-->
<!--    console.log("查到了K线数据",res);-->
<!--    // myChart.hideLoading();-->
<!--  } catch (err) {-->
<!--    console.log(err);-->
<!--  }-->
<!--  // invoke<StockData[]>('query_stocks_day_k_limit',{code:code}).then((res)=>{-->
<!--  //   // console.log(res)-->
<!--  //   // data.value = handleRawData(res)-->
<!--  //   console.log("查到了数据")-->
<!--  //   rawData.value = res-->
<!--  // }).catch((err)=>{-->
<!--  //   console.log(err)-->
<!--  // })-->
<!--}-->
<!--function handleRawData(raw: StockData[]){-->
<!--  let categoryData = [];-->
<!--  let values = [];-->
<!--  let volumes = [];-->
<!--  let ma5 = [];-->
<!--  let ma10 = [];-->
<!--  let ma20 = [];-->
<!--  let ma60 = [];-->
<!--  for (let i = 0; i < raw.length; i++) {-->
<!--    let element = raw[i];-->
<!--    let color, borderColor;-->
<!--    if (element.open < element.close) {-->
<!--      color = 'white';-->
<!--      borderColor = 'red';-->
<!--    } else {-->
<!--      color = 'green';-->
<!--      borderColor = 'green';-->
<!--    }-->
<!--    categoryData.push(element.date);-->
<!--    values.push([element.open,element.close,element.low,element.high,element.vol]);-->
<!--    // volumes.push([i, element.vol, element.open > element.close ? 1 : -1]);-->
<!--    volumes.push({-->
<!--      value:element.vol,-->
<!--      itemStyle: {-->
<!--        color:color,-->
<!--        borderColor:borderColor-->
<!--      }-->
<!--    });-->
<!--    ma5.push(element.ma5);-->
<!--    ma10.push(element.ma10);-->
<!--    ma20.push(element.ma20);-->
<!--    ma60.push(element.ma60);-->
<!--  }-->
<!--  return {-->
<!--    categoryData: categoryData,-->
<!--    values: values,-->
<!--    volumes: volumes,-->
<!--    ma5: ma5,-->
<!--    ma10: ma10,-->
<!--    ma20:ma20,-->
<!--    ma60:ma60-->
<!--  };-->
<!--}-->
<!--function scrollEvent(deltaY:number){-->
<!--  let index = store.stockinfoGs.findIndex((item)=>item.code==store.stockinfoG?.code)-->
<!--  if (index !== -1) {-->
<!--    if (deltaY > 0) {-->
<!--      // 向下滚动-->
<!--      if (index === store.stockinfoGs.length - 1) {-->
<!--        // 如果当前是最后一个元素，则回到第一个元素-->
<!--        store.stockinfoG = store.stockinfoGs[0];-->
<!--      } else {-->
<!--        // 否则滚动到下一个元素-->
<!--        store.stockinfoG = store.stockinfoGs[index + 1];-->
<!--      }-->
<!--    } else if (deltaY < 0) {-->
<!--      // 向上滚动-->
<!--      if (index === 0) {-->
<!--        // 如果当前是第一个元素，则回到最后一个元素-->
<!--        store.stockinfoG = store.stockinfoGs[store.stockinfoGs.length - 1];-->
<!--      } else {-->
<!--        // 否则滚动到上一个元素-->
<!--        store.stockinfoG = store.stockinfoGs[index - 1];-->
<!--      }-->
<!--    }-->
<!--    console.log("滚动了,改变code")-->
<!--    code = store.stockinfoG!.code;-->
<!--    // myChart.showLoading();-->
<!--    query_stocks_day_k_limit();-->
<!--    query_graphic();-->
<!--  } else {-->
<!--    console.log('stockinfoG不在stockinfoGs数组中');-->
<!--  }-->
<!--}-->

<!--let paintState: PaintState = PaintState.Null;-->

<!--function handlePaint(state: PaintState){-->
<!--  paintState = state;-->
<!--  if (state==PaintState.Null){-->
<!--    myChart.getZr().off('click', clickHandler);-->
<!--    myChart.getZr().off('mousehover', hoverHandler);-->
<!--  }else {-->
<!--    console.log("进入绘制hl")-->
<!--    if (state==PaintState.HLS){-->
<!--      console.log("进入绘制hls")-->
<!--      myChart.getZr().on('mousemove', hoverHandler);-->
<!--    }-->
<!--    myChart.getZr().on('click', clickHandler);-->
<!--  }-->
<!--}-->
<!--let number = "first";-->
<!--const newLineR = ref()-->
<!--const hoverHandler = function (params: any) {-->
<!--  const pointInPixel = [params.offsetX, params.offsetY];-->
<!--  if (myChart.containPixel('grid',pointInPixel)) {-->
<!--    let hoverPoint = myChart.convertFromPixel({seriesIndex: 0}, pointInPixel);-->
<!--    console.log("在移动")-->
<!--    if (number=="first"){-->
<!--    }else if (number=="second"){-->
<!--      newLineR.value.end = [hoverPoint[0],hoverPoint[1]];-->
<!--    }-->
<!--  }-->
<!--};-->
<!--const clickHandler = function (params: any) {-->
<!--  console.log("绘制状态",paintState)-->
<!--  const pointInPixel = [params.offsetX, params.offsetY];-->
<!--  if (myChart.containPixel('grid',pointInPixel)) {-->
<!--    let clickPoint = myChart.convertFromPixel({seriesIndex: 0}, pointInPixel)-->
<!--    if (number=="first"){-->
<!--      console.log("绘制第一个点")-->
<!--      // lineData.value.push({id:generateId(), start: [clickPoint[0],clickPoint[1]], end: [clickPoint[0],clickPoint[1]],type:paintState});-->
<!--      let newLine = {id:generateId(), start: [clickPoint[0],clickPoint[1]], end: [clickPoint[0],clickPoint[1]],type:paintState};-->
<!--      newLineR.value = newLine;-->
<!--      number = "second"-->
<!--      lineData.value.push(newLineR.value);-->
<!--    }else if (number=="second"){-->
<!--      console.log("绘制第2个点")-->
<!--      newLineR.value.end = [clickPoint[0],clickPoint[1]];-->
<!--      number = "first";-->
<!--      handlePaint(PaintState.Null);-->
<!--    }-->
<!--  }-->
<!--};-->
<!--function updateLineOption(){-->
<!--  myChart.setOption({-->
<!--    graphic: lineData.value.map(function (item, dataIndex) {-->
<!--      if (item.type===PaintState.HL||item.type===PaintState.HLS){-->
<!--        let pixelElement = myChart.convertToPixel({seriesIndex: 0}, item.start);-->
<!--        console.log("绘制线",pixelElement)-->
<!--        return {-->
<!--          type:'group',-->
<!--          x: pixelElement[0],-->
<!--          y: pixelElement[1]-20,-->
<!--          draggable: true,-->
<!--          children:[-->
<!--            {-->
<!--              id:item.id,-->
<!--              type: 'line',-->
<!--              shape: {-->
<!--                x1: 0,-->
<!--                //x1: pixelElement[0],-->
<!--                // y1: myChart.convertToPixel({seriesIndex: 0}, item.start)[1],-->
<!--                y1: 20,-->
<!--                x2: myChart.convertToPixel({seriesIndex: 0}, item.end)[0]-pixelElement[0],-->
<!--                y2: myChart.convertToPixel({seriesIndex: 0}, item.end)[1]-pixelElement[1]+20,-->
<!--                percent: 100-->
<!--              },-->
<!--              z:100,-->
<!--            },-->
<!--            {-->
<!--              type: 'text',-->
<!--              style: {-->
<!--                text: item.start[1],-->
<!--                fill: '#ecc032',-->
<!--                font: '14px Microsoft YaHei'-->
<!--              },-->
<!--              textConfig: {-->
<!--                // inside:true,-->
<!--                position: 'insideTopLeft',-->
<!--              },-->
<!--            }-->
<!--          ],-->
<!--          ondrag: function (param) {-->
<!--            console.log(dataIndex, [this.x, this.y]);-->
<!--            let numbers = myChart.convertFromPixel({seriesIndex: 0}, [this.x, this.y+20]);-->
<!--            let diffx = numbers[0]-lineData.value[dataIndex].start[0];-->
<!--            let diffy = numbers[1]-lineData.value[dataIndex].start[1];-->
<!--            //打印这两个diff-->
<!--            console.log("diff",diffx,diffy);-->
<!--            //计算ponit与lineData.value[dataIndex].start的差值-->
<!--            // let diff = [numbers[0]-lineData.value[dataIndex].start[0],numbers[1]-lineData.value[dataIndex].start[1]];-->
<!--            lineData.value.forEach(function (item2, index) {-->
<!--              if (item2.id===item.id){-->
<!--                console.log("进来改变")-->
<!--                item2.end = [lineData.value[dataIndex].end[0]+diffx,lineData.value[dataIndex].end[1]+diffy];-->

<!--                item2.start = [numbers[0],numbers[1]];-->
<!--                console.log("改变",item2.start)-->
<!--              }-->
<!--            })-->
<!--            console.log(param);-->
<!--          },-->
<!--        }-->
<!--      }-->
<!--    })-->
<!--  })-->
<!--}-->
<!--function init_option(){-->
<!--  const data = handleRawData(rawData.value);-->
<!--  return {-->
<!--    animation: false,-->
<!--    legend: {-->
<!--      top: 10,-->
<!--      // left: 'left',-->
<!--      left: '4%',-->
<!--      data: ['K线', 'MA5', 'MA10', 'MA20', 'MA60']-->
<!--      // data: ['K线', ]-->
<!--    },-->
<!--    tooltip: {-->
<!--      // trigger: 'axis',// https://echarts.apache.org/zh/option.html#grid.tooltip.trigger-->
<!--      trigger: 'item',-->
<!--      axisPointer: {-->
<!--        type: 'cross',-->
<!--        animation:false-->
<!--        // snap:true 坐标轴指示器是否自动吸附到点上。默认自动判断。好像是默认启用 https://echarts.apache.org/zh/option.html#grid.tooltip.axisPointer.snap-->
<!--      },-->
<!--      transitionDuration: 0,-->
<!--      borderWidth: 1,-->
<!--      borderColor: '#776d6d',-->
<!--      backgroundColor : 'rgba(50,50,50,0)',-->
<!--      padding: 5,-->
<!--      textStyle: {-->
<!--        color: '#000'-->
<!--      },-->
<!--      formatter: function (params: any) { // params 是 formatter 需要的数据集-->
<!--        let htmlContent;-->
<!--        if (params.seriesType == "candlestick") {-->
<!--          let change = (params.data[2] - params.data[1]).toFixed(3);-->
<!--          const { openClass, closeClass, changeClass } = formatPriceLabel(params.data[1], params.data[2], change);-->
<!--          htmlContent = `<div class="column tip">-->
<!--      <label style="background-color: rgb(241, 241, 148)">${params.name}/${computeWeek(params.name)}</label>-->
<!--      <hr style="margin:0;"> &lt;!&ndash; 添加水平线并设置上下边距 &ndash;&gt;-->
<!--      <label class="${openClass}">开盘：${params.data[1]}</label>-->
<!--      <label class="red-label">最高：${params.data[4]}</label>-->
<!--      <label class="green-label">最低：${params.data[3]}</label>-->
<!--      <label class="${closeClass}">收盘：${params.data[2]}</label>-->
<!--      <label class="${changeClass}">涨跌：${change}</label>-->
<!--      <label class="${changeClass}">涨幅：${calculateChangeRate(params.data[1], params.data[2])}%</label>-->
<!--&lt;!&ndash;      <label>成交量：${params.data[5]}</label>&ndash;&gt;-->
<!--      <label>成交量：${formatLargeNumber(params.data[5])}</label>-->
<!--    </div>`-->
<!--        } else if (params.seriesType == "line") {-->
<!--          if (params.seriesName == "MA5") {-->
<!--            htmlContent = `<label>MA(MA5):${params.value}</label>`-->
<!--          } else if (params.seriesName == "MA10") {-->
<!--            htmlContent = `<label>MA(MA10):${params.value}</label>`-->
<!--          } else if (params.seriesName == "MA20") {-->
<!--            htmlContent = `<label>MA(MA20):${params.value}</label>`-->
<!--          } else if (params.seriesName == "MA30") {-->
<!--            htmlContent = `<label>MA(MA30):${params.value}</label>`-->
<!--          } else if (params.seriesName == "MA60") {-->
<!--            htmlContent = `<label>MA(MA60):${params.value}</label>`-->
<!--          }-->
<!--        }-->
<!--        return htmlContent-->
<!--      }-->
<!--    },-->
<!--    axisPointer: {-->
<!--      link: [-->
<!--        {-->
<!--          xAxisIndex: 'all'-->
<!--        }-->
<!--      ],-->
<!--      label: {-->
<!--        backgroundColor: '#e1bf2c',-->
<!--        color:"black"-->
<!--      }-->
<!--    },-->
<!--    toolbox: {-->
<!--      show: true,-->
<!--      feature: {-->
<!--        myTool2: {-->
<!--          show: true,-->
<!--          title: '画线',-->
<!--          // icon: 'image://https://echarts.apache.org/zh/images/favicon.png',-->
<!--          icon: 'path://M811.562667 348.202667l78.848-78.848a10.666667 10.666667 0 0 0 0-15.082667l-120.682667-120.682667a10.666667 10.666667 0 0 0-15.082667 0l-78.848 78.848 135.765334 135.765334z m-45.248 45.248l-135.765334-135.765334L212.053333 676.181333 171.306667 852.693333l176.512-40.725333L766.293333 393.450667zM814.997333 88.32l120.682667 120.682667a74.666667 74.666667 0 0 1 0 105.6L386.56 863.701333a32 32 0 0 1-15.424 8.533334L135.829333 926.570667a32 32 0 0 1-38.4-38.378667l54.314667-235.306667a32 32 0 0 1 8.554667-15.445333L709.397333 88.32a74.666667 74.666667 0 0 1 105.6 0z',-->
<!--          onclick: function (){-->
<!--            showPaintTool();-->
<!--          }-->
<!--        },-->
<!--        restore: {},-->
<!--        saveAsImage: {}-->
<!--      }-->
<!--    },-->
<!--    visualMap: {-->
<!--      show: false,-->
<!--      seriesIndex: 5,-->
<!--      dimension: 2,-->
<!--      pieces: [-->
<!--        {-->
<!--          value: 1,-->
<!--          color: downColor-->
<!--        },-->
<!--        {-->
<!--          value: -1,-->
<!--          color: bColor-->
<!--        }-->
<!--      ],-->
<!--    },-->
<!--    grid: [-->
<!--      {-->
<!--        left: '4%',-->
<!--        right: '3%',-->
<!--        height: '65%'-->
<!--      },-->
<!--      {-->
<!--        left: '4%',-->
<!--        right: '3%',-->
<!--        top: '75%',-->
<!--        height: '15%'-->
<!--      }-->
<!--    ],-->
<!--    xAxis: [-->
<!--      {-->
<!--        type: 'category',-->
<!--        data: data.categoryData,-->
<!--        boundaryGap: false,-->
<!--        axisLine: {onZero: false},-->
<!--        splitLine: {show: false},-->
<!--        min: 'dataMin',-->
<!--        max: 'dataMax',-->
<!--        axisPointer: {-->
<!--          z: 100-->
<!--        }-->
<!--      },-->
<!--      {-->
<!--        type: 'category',-->
<!--        gridIndex: 1,-->
<!--        data: data.categoryData,-->
<!--        boundaryGap: false,-->
<!--        axisLine: {onZero: false},-->
<!--        axisTick: {show: false},-->
<!--        splitLine: {show: false},-->
<!--        axisLabel: {show: true},-->
<!--        min: 'dataMin',-->
<!--        max: 'dataMax'-->
<!--      }-->
<!--    ],-->
<!--    yAxis: [-->
<!--      {-->
<!--        scale: true,-->
<!--        splitArea: {-->
<!--          show: true-->
<!--        }-->
<!--      },-->
<!--      {-->
<!--        scale: true,-->
<!--        gridIndex: 1,-->
<!--        splitNumber: 2,-->
<!--        axisLabel: {show: false},-->
<!--        axisLine: {show: false},-->
<!--        axisTick: {show: false},-->
<!--        splitLine: {show: false}-->
<!--      }-->
<!--    ],-->
<!--    dataZoom: [-->
<!--      {-->
<!--        type: 'inside',-->
<!--        xAxisIndex: [0, 1],-->
<!--        start: 90,-->
<!--        end: 100,-->
<!--        zoomOnMouseWheel: "ctrl",// 启用鼠标滚轮触发缩放-->
<!--        zoomLock: true-->
<!--      },-->
<!--      {-->
<!--        show: true,-->
<!--        xAxisIndex: [0, 1],-->
<!--        type: 'slider',-->
<!--        top: '93%',-->
<!--        start: 85,-->
<!--        end: 100-->
<!--      }-->
<!--    ],-->
<!--    series: [-->
<!--      {-->
<!--        name: 'K线',-->
<!--        type: 'candlestick',-->
<!--        data: data.values,-->
<!--        itemStyle: {-->
<!--          color: upColor,-->
<!--          color0: downColor,-->
<!--          borderColor: bColor,-->
<!--          borderColor0: undefined-->
<!--        },-->
<!--        //https://echarts.apache.org/zh/option.html#series-candlestick-->
<!--        //https://echarts.apache.org/zh/option.html#series-line.markPoint.data.valueDim-->
<!--        markPoint: {-->
<!--          data:[-->
<!--            {-->
<!--              type: 'max',-->
<!--              name: '最大值',-->
<!--              valueDim: 'highest'-->
<!--            },-->
<!--            {-->
<!--              type: 'min',-->
<!--              name: '最小值',-->
<!--              valueDim: 'lowest'-->
<!--            }-->
<!--          ],-->
<!--          symbol: "arrow",-->
<!--          symbolSize: 30,-->
<!--          silent: true,-->
<!--          itemStyle:{-->
<!--            color:"#ecece400"-->
<!--          },-->
<!--          label:{-->
<!--            color:"blue"-->
<!--          }-->
<!--        }-->
<!--      },-->
<!--      {-->
<!--        name: 'MA5',-->
<!--        type: 'line',-->
<!--        data: data.ma5,-->
<!--        smooth: true,-->
<!--        // symbol:'none',-->
<!--        symbolSize: 0,-->
<!--        lineStyle: {-->
<!--          opacity: 0.5-->
<!--        },-->
<!--      },-->
<!--      {-->
<!--        name: 'MA10',-->
<!--        type: 'line',-->
<!--        data: data.ma10,-->
<!--        smooth: true,-->
<!--        // symbol:'none',-->
<!--        symbolSize: 0,-->
<!--        lineStyle: {-->
<!--          opacity: 0.5-->
<!--        }-->
<!--      },-->
<!--      {-->
<!--        name: 'MA20',-->
<!--        type: 'line',-->
<!--        // symbol:'none',-->
<!--        symbolSize: 0,-->
<!--        data: data.ma20,-->
<!--        smooth: true,-->
<!--        lineStyle: {-->
<!--          opacity: 0.5-->
<!--        }-->
<!--      },-->
<!--      {-->
<!--        name: 'MA60',-->
<!--        type: 'line',-->
<!--        // symbol:'none',-->
<!--        symbolSize: 0,-->
<!--        data: data.ma60,-->
<!--        smooth: true,-->
<!--        lineStyle: {-->
<!--          opacity: 0.5-->
<!--        },-->
<!--      },-->
<!--      {-->
<!--        name: 'Volume',-->
<!--        type: 'bar',-->
<!--        xAxisIndex: 1,-->
<!--        yAxisIndex: 1,-->
<!--        data: data.volumes,-->
<!--      },-->
<!--    ],-->
<!--  }-->
<!--}-->
<!--// function updatePosition() {-->
<!--//   console.log("更新点了")-->
<!--//   myChart.setOption({-->
<!--//     graphic: dataItem.map(function (item, dataIndex) {-->
<!--//       return {-->
<!--//         id: 'text1',-->
<!--//         shape: {-->
<!--//           x1: myChart.convertToPixel({seriesIndex: 0}, item[0])[0],-->
<!--//           y1: myChart.convertToPixel({seriesIndex: 0}, item[0])[1],-->
<!--//           x2: myChart.convertToPixel({seriesIndex: 0}, item[1])[0],-->
<!--//           y2: myChart.convertToPixel({seriesIndex: 0}, item[1])[1],-->
<!--//           percent: 100-->
<!--//         },-->
<!--//       };-->
<!--//     })-->
<!--//   });-->
<!--// }-->
<!--function computeWeek(date:string){-->
<!--  // console.log(date)-->
<!--  var date = new Date(date)-->
<!--  var day = date.getDay();-->
<!--  // 根据获取的数字，转换为对应的中文星期几-->
<!--  var weekDays = ["日", "一", "二", "三", "四", "五", "六"];-->
<!--  var weekDay = weekDays[day];-->

<!--  // console.log(weekDay); // 输出：周四-->
<!--  return weekDay-->
<!--}-->
<!--function formatPriceLabel(open:number, close:number, change:number) {-->
<!--  let openClass = 'black-label';-->
<!--  let closeClass = 'black-label';-->
<!--  let changeClass = 'black-label';-->
<!--  if (open < close) {-->
<!--    openClass = 'green-label';-->
<!--    closeClass = 'red-label';-->
<!--  } else if (open > close) {-->
<!--    openClass = 'red-label';-->
<!--    closeClass = 'green-label';-->
<!--  }-->
<!--  if (change > 0) {-->
<!--    changeClass = 'red-label'; // 涨幅高于0，红色-->
<!--  } else if (change < 0) {-->
<!--    changeClass = 'green-label';   // 涨幅低于0，绿色-->
<!--  }-->
<!--  return {-->
<!--    openClass: openClass,-->
<!--    closeClass: closeClass,-->
<!--    changeClass: changeClass-->
<!--  };-->
<!--}-->
<!--function formatLargeNumber(num:number) {-->
<!--  // 判断是否为数字-->
<!--  if (isNaN(num)) {-->
<!--    return '非数字';-->
<!--  }-->
<!--  // 转换并格式化数字  423721924-->
<!--  let formatted;-->
<!--  if (num >= 1e8) { // 亿-->
<!--    formatted = (num / 1e8).toFixed(2) + '亿';-->
<!--  } else if (num >= 1e4) { // 万-->
<!--    formatted = (num / 1e4).toFixed(2) + '万';-->
<!--  } else { // 千以下直接显示-->
<!--    formatted = num.toString();-->
<!--  }-->
<!--  return formatted;-->
<!--}-->
<!--function calculateChangeRate(openPrice:number, closePrice:number) {-->
<!--  // 计算涨跌幅-->
<!--  const changeRate = ((closePrice - openPrice) / openPrice) * 100;-->
<!--  // 保留两位小数并返回-->
<!--  return changeRate.toFixed(2);-->
<!--}-->
<!--async function showPaintTool(){-->
<!--  const appWindow = WebviewWindow.getByLabel('tool')-->
<!--  await appWindow?.show()-->
<!--}-->
<!--</script>-->

<!--<template>-->
<!--  <div id="main" ref="chart" class="candle-chart-container" style="border: #9d6a09 1px solid "></div>-->

<!--&lt;!&ndash;  <div class="candle-chart-container">&ndash;&gt;-->
<!--&lt;!&ndash;  <el-button @click="back()">退回</el-button>&ndash;&gt;-->
<!--&lt;!&ndash;  <div v-shortkey.once="['f10']"  @shortkey="handleKeydown()">&ndash;&gt;-->
<!--&lt;!&ndash;    &lt;!&ndash;  <div >&ndash;&gt;&ndash;&gt;-->
<!--&lt;!&ndash;    &lt;!&ndash; 为 ECharts 准备一个定义了宽高的 DOM &ndash;&gt;&ndash;&gt;-->
<!--&lt;!&ndash;    <div id="main" ref="chart" class="candle-chart-container" style="border: #9d6a09 1px solid "></div>&ndash;&gt;-->
<!--&lt;!&ndash;    &lt;!&ndash;    <input ref="tip" style="width: 60px;font-size: 12px;visibility: hidden">&ndash;&gt;&ndash;&gt;-->
<!--&lt;!&ndash;    <input v-if="tipConfig.isVisible" ref="tip" :style="{&ndash;&gt;-->
<!--&lt;!&ndash;      width: `60px`,&ndash;&gt;-->
<!--&lt;!&ndash;      fontSize:`11px`,&ndash;&gt;-->
<!--&lt;!&ndash;        position: 'absolute',&ndash;&gt;-->
<!--&lt;!&ndash;        // position: 'relative',&ndash;&gt;-->
<!--&lt;!&ndash;        left: `${tipConfig.x}px`,&ndash;&gt;-->
<!--&lt;!&ndash;        top: `${tipConfig.y}px`,&ndash;&gt;-->
<!--&lt;!&ndash;        // visibility: tipConfig.isVisible ? 'visible' : 'hidden'&ndash;&gt;-->
<!--&lt;!&ndash;      }"&ndash;&gt;-->
<!--&lt;!&ndash;           @keydown.enter = "addTip()"&ndash;&gt;-->
<!--&lt;!&ndash;           @focus = "focusChange()"&ndash;&gt;-->
<!--&lt;!&ndash;    >&ndash;&gt;-->
<!--&lt;!&ndash;  </div>&ndash;&gt;-->
<!--&lt;!&ndash;  </div>&ndash;&gt;-->
<!--</template>-->

<!--<style >-->
<!--.candle-chart-container{-->
<!--  width: 85%;-->
<!--  height: 100%;-->
<!--  border: #535bf2 1px solid;-->
<!--}-->
<!--.tip{-->
<!--  font-family:"Adobe 黑体 Std R",serif;-->
<!--  font-size: 13px;-->
<!--  border-color: #24c8db00;-->
<!--  border-width: 1px;-->
<!--  border-style: solid; /* 添加边框样式 */-->
<!--  width: 110px;-->
<!--  background-color: #ecece498;-->
<!--}-->
<!--.red-label {-->
<!--  color: red;-->
<!--}-->

<!--.green-label {-->
<!--  color: green;-->
<!--}-->

<!--.black-label {-->
<!--  color: black;-->
<!--}-->
<!--</style>-->