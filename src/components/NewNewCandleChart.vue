<script lang="ts" setup>
import {invoke} from "@tauri-apps/api/core";
import {nextTick, onMounted, ref, watch} from "vue";
import * as echarts from "echarts/core";
import {Graphic, PaintState, StockData, StockLiveData} from "../type.ts";
import {WebviewWindow} from "@tauri-apps/api/webviewWindow";
import {EChartsType} from "echarts";
import {store} from "../store.ts";
import {debounce, errorNotification, generateId, successNotification} from "../utils.ts"
import {emit, listen, TauriEvent, UnlistenFn} from "@tauri-apps/api/event";
import {onBeforeRouteLeave, useRouter} from "vue-router";
import InlineSvg from "vue-inline-svg";

interface VolumeItem {
  value: number;
  itemStyle: {
    color: string;
    borderColor: string;
  };
}

interface CandleStockData {
  categoryData: string[];
  values: [number, number, number, number, number][];
  volumes: VolumeItem[];
  ma5: number[];
  ma10: number[];
  ma20: number[];
  ma60: number[];
}

let isCtrlPressed = false;

const router = useRouter()

router.afterEach(async (to) => {
  // 这段代码会在路由跳转完成后执行
  // 注意：它不会等待组件挂载完成，也不提供组件实例的访问
  if (myChart!=undefined&&to.fullPath.includes("detail")){
    unlistenResize = await listen(TauriEvent.WINDOW_RESIZED, async () => {
      myChart.resize();
      updateLineOption();
    });
    unlistenPaint = await listen('paint', (event) => {
      handlePaint(event.payload.state);
    })
    unlistenLiveData = await listen("live_stock_data", ({payload }) => {
      if (chartIsInit){
        updateLiveData(payload);
      }
    })
    document.addEventListener('keydown', unlockZoom);
    document.addEventListener('keyup', lockZoom);
    document.addEventListener('wheel',wheelChangeCode);
  }
});

let unlistenPaint:UnlistenFn;
let unlistenResize:UnlistenFn;
let unlistenLiveData:UnlistenFn;
const debouncedFunction = debounce(scrollEvent, 700, true);

let code = store.stockinfoG!.code;
watch(() => store.stockinfoG,async (newValue: any) => {
  if(code!=newValue.code){
    save_graphic();
    code = newValue.code;
    // console.log("info code变化了",code);
    await updateChart();
    // myChart.setOption({})
  }
},{deep:true})


const upColor = 'rgba(255,255,255,0.6)';
const bColor = '#ec0000';
const downColor = 'rgb(55,150,55)';
const nowDate = getFormattedDate();

const contextMenuShow = ref(false);


const chart=ref(null);
let chartIsInit = false;
let myChart: EChartsType;
const rawData = ref([]);
const graphicData = ref<Graphic[]>([])
let newGraphicData:Graphic[] =[]
let stockData:CandleStockData;
// 创建一个映射来存储每个id对应的group
const groupMap = new Map<string, { group: any }>();

const inputVisible = ref(false)
const inputValue = ref('');
const inputRef = ref();
const newTextColor = ref("#FF6A00")
const newTextFontSize = ref(16)

const options = {
  // theme: 'win10 dark',
  id:"",
  group_id:"",
  zIndex: 3,
  x: 500,
  y: 200
}
watch(()=>store.isBlur,(newValue)=>{
  if (newValue){
    //当页面获得焦点时，锁定缩放图表，并将isCtrlPressed改为false。因为其他快捷键比如qq截图，会按下Ctrl（此时会解锁图表缩放），然后页面失去焦点无法
    //监听到Ctrl释放，会导致图表缩放状态异常。
    contextMenuShow.value = false;
    setZoomLock(true);
    isCtrlPressed = false;
  }
})

watch(graphicData, (_) => {
  // console.log("lineData变化了",graphicData.value);
  // handleGraphicData();
  updateLineOption()
},{deep:true});

onMounted(async ()=>{
  myChart = echarts.init(chart.value);
  // await query_stocks_day_k_limit();
  // await query_graphic();
  await updateChart();
  myChart.on('dataZoom', updateLineOption);
  unlistenPaint = await listen('paint', (event) => {
    // console.log("开启绘制1")
    handlePaint(event.payload.state);
  })
  unlistenLiveData = await listen("live_stock_data", ({payload }) => {
    // console.log("K线图也受到了推送的数据")
    if (chartIsInit){ //https://www.cnblogs.com/boke-biji123/p/15514457.html
      updateLiveData(payload);
    }
  })
  unlistenResize = await listen(TauriEvent.WINDOW_RESIZED, async (param) => {
    // console.log("蜡烛图1监听到了窗口尺寸改变",param);
    if (param.payload.height>50){
      await nextTick();
      // myChart.resize();
      myChart.resize();
      updateLineOption();
    }
  });
  // unlistenResize();
  // myChart.on('dataZoom', updateLineOption);
  // const debouncedFunction = debounce(scrollEvent, 700, true);
  // document.addEventListener('wheel', (event: WheelEvent) => {
  //   console.log("deltaY",event.deltaY)
  //   // 这里可以获取滚动事件的信息
  //   const deltaY = event.deltaY; // 滚动的垂直距离
  //   debouncedFunction(deltaY)
  // });
  document.addEventListener('wheel',wheelChangeCode);
  document.addEventListener('keydown', unlockZoom);
  document.addEventListener('keyup', lockZoom);

  // document.addEventListener('keydown', (event: KeyboardEvent) => {
  //   if (event.ctrlKey) {
  //     if (!isCtrlPressed){
  //       isCtrlPressed = true;
  //       console.log("解锁图表缩放")
  //       myChart.setOption({
  //         dataZoom: [
  //           {
  //             type: 'inside',
  //             xAxisIndex: [0, 1],
  //             zoomLock: false
  //           },
  //         ],
  //       })
  //     }
  //   }
  // });
  // document.addEventListener('keyup', (event: KeyboardEvent) => {
  //   if (event.key === 'Control') {
  //     console.log("锁定图表缩放")
  //     if (isCtrlPressed){
  //       isCtrlPressed = false;
  //       myChart.setOption({
  //         dataZoom: [
  //           {
  //             type: 'inside',
  //             xAxisIndex: [0, 1],
  //             zoomLock: true
  //           },
  //         ],
  //       })
  //     }
  //   }
  // });
})
onBeforeRouteLeave(async function (){
  console.log("离开蜡烛图路由");
  // myChart.off('dataZoom', updateLineOption);
  save_graphic();
  unlistenResize();
  unlistenLiveData();
  unlistenPaint();
  document.removeEventListener('wheel',wheelChangeCode);
  document.removeEventListener('keydown', unlockZoom);
  document.removeEventListener('keyup', lockZoom);
  // window.removeEventListener('resize', calculateTableHeight);
})
function handleRawData(raw: StockData[]){
  let categoryData = [];
  let values = [];
  let volumes = [];
  let ma5 = [];
  let ma10 = [];
  let ma20 = [];
  let ma60 = [];
  let element = raw[0];
  let color, borderColor;
  if (element.open < element.close) {
    color = 'white';
    borderColor = 'red';
  } else {
    color = 'green';
    borderColor = 'green';
  }
  categoryData.push(element.date);
  values.push([element.open,element.close,element.low,element.high,element.vol,0.0]);
  // volumes.push([i, element.vol, element.open > element.close ? 1 : -1]);
  volumes.push({
    value:element.vol,
    itemStyle: {
      color:color,
      borderColor:borderColor
    }
  });
  ma5.push(element.ma5);
  ma10.push(element.ma10);
  ma20.push(element.ma20);
  ma60.push(element.ma60);
  for (let i = 1; i < raw.length; i++) {
    let element = raw[i];
    let color, borderColor;
    if (element.open < element.close) {
      color = 'white';
      borderColor = 'red';
    } else {
      color = 'green';
      borderColor = 'green';
    }
    categoryData.push(element.date);
    //最后一个元素是涨跌幅，需要用到前一天的收盘价
    values.push([element.open,element.close,element.low,element.high,element.vol,calculateChangeRate(raw[i-1].close,element.close)]);
    // volumes.push([i, element.vol, element.open > element.close ? 1 : -1]);
    volumes.push({
      value:element.vol,
      itemStyle: {
        color:color,
        borderColor:borderColor
      }
    });
    ma5.push(element.ma5);
    ma10.push(element.ma10);
    ma20.push(element.ma20);
    ma60.push(element.ma60);
  }
  stockData = {
    categoryData: categoryData,
    values: values,
    volumes: volumes,
    ma5: ma5,
    ma10: ma10,
    ma20:ma20,
    ma60:ma60
  }
  return stockData;
}
function handleGraphicData(graphics: Graphic[]=graphicData.value){
  // let graphics = lineData.value;
  graphics.forEach((item) => {
    let start = myChart?.convertToPixel({seriesIndex: 0}, item.start);
    if (!start) {
      return; // 如果转换失败，则跳过当前项
    }
    const groupId = item.group_id;
    let style = item.style;
    let groupEntry = groupMap.get(groupId);
    let graphic;
    if (item.graphic_type === 'line') {
      let end = myChart?.convertToPixel({seriesIndex: 0}, item.end);
      graphic = {
        type: 'line',
        id: item.id, // 使用graphic_id或id作为line的id
        // $action: 'replace',
        shape: {
          x1: start[0],
          y1: start[1],
          x2: end[0],
          y2: end[1],
        },
        style: {
          lineWidth: style?.line_width?style?.line_width:1,
          fill: style?.color,
        },
        onmouseup: function (param:any) {
          if (param.event.button==2){
            options.x=param.event.offsetX+10;
            options.y=param.event.offsetY+10;
            options.id=item.id;
            options.group_id=item.group_id;
            contextMenuShow.value = true;
          }
          param.event.stopPropagation();
        }
      }
    }else if (item.graphic_type === 'text') {
      let thisLine:Graphic;
      if (item.content==null){
        thisLine = graphics.filter(function (item2) {
          return (item2.group_id===item.group_id&&item2.graphic_type=="line")
        })[0];
      }
      graphic = {
        type:"text",
        id: item.id,
        position: [start[0], start[1]],
        style: {
          fill: style?.color?style?.color:'black',
          // fontsize: style?.size?`${style?.size}rem`:'15rem',
          fontSize:style?.size?style?.size:14,
          lineWidth: style?.line_width?style?.line_width:1,
          // text: item.content?item.content:item.start[1].toFixed(3),
          text: item.content?item.content:thisLine.start[1].toFixed(3),
        },
        draggable: true,
        cursor: 'move',
        onmouseup: function (param:any) {
          if (param.event.button==2){
            options.x=param.event.offsetX+10;
            options.y=param.event.offsetY+10;
            options.id=item.id;
            options.group_id=item.group_id;
            contextMenuShow.value = true;
          }
          param.event.stopPropagation();
        },
        ondragend: function () {
          let newPointData = myChart.convertFromPixel({seriesIndex: 0}, [this.x, this.y]);
          graphicData.value.filter(function (item2) {
            if (item2.id===item.id){
              item2.start = newPointData;
            }
          })
        }
      }
    }
    if (!groupEntry) {
      let startPointData:number[];
      // 如果没有找到对应的group，则创建一个新的group和lines数组
      groupEntry = {
        group: {
          type: 'group',
          id: item.group_id,
          draggable: true,
          children: [graphic],
          ondragstart: function () {
            startPointData = myChart.convertFromPixel({seriesIndex: 0}, [this.x, this.y]);
          },
          ondragend: function () {
            let endPointData = myChart.convertFromPixel({seriesIndex: 0}, [this.x, this.y]);
            let diff = [endPointData[0]-startPointData[0],endPointData[1]-startPointData[1]];
            graphicData.value.forEach(function (item2){
              if (item2.group_id===item.group_id){
                item2.start = [item2.start[0]+diff[0],item2.start[1]+diff[1]];
                item2.end = [item2.end[0]+diff[0],item2.end[1]+diff[1]];
              }
            });
            //一定要重新设置group的位置，不然会造成孩子定位异常！！
            this.x = 0;
            this.y = 0;
          }
        }
      };
      groupMap.set(groupId, groupEntry);
    } else {
      groupEntry.group.children.push(graphic)
    }
  });
  myChart.setOption({
    graphic:Array.from(groupMap.values()).map(function (item) {
      return item.group;
    })
  });
  groupMap.clear();//处理完就清空，还要接着用
}

function handleNewGraphicData(graphics: Graphic[]=newGraphicData){
  handleGraphicData(graphics);
  graphicData.value.push(...graphics);
  newGraphicData = [];
}
async function updateChart(){
  clear_all();
  await query_stocks_day_k_limit();
  await query_graphic();
}
function updateLiveData(live_data:Record<string, StockLiveData>){
  // console.log("更新实时数据",live_data[code],live_data[code] != undefined)
  if (live_data[code] != undefined){
    const newData = live_data[code];
    let len = stockData.categoryData.length-1;
    let color, borderColor;
    if (newData.open < newData.price) {
      color = 'white';
      borderColor = 'red';
    } else {
      color = 'green';
      borderColor = 'green';
    }
    if(stockData.categoryData[len]!=nowDate){
      // console.log("不是一天的数据，更新");
      stockData.categoryData.push(nowDate);
      stockData.values.push([newData.open,newData.price,newData.low,newData.high,newData.volume]);
      // stockData.volumes.push([newData.volume, newData.open > newData.price ? 1 : -1]);
      stockData.volumes.push({
        value:newData.volume,
        itemStyle: {
          color:color,
          borderColor:borderColor
        }
      });
      stockData.ma5.push(newData.ma5);
      stockData.ma10.push(newData.ma10);
      stockData.ma20.push(newData.ma20);
      stockData.ma60.push(newData.ma60);
    }else{
      stockData.values[len] = [newData.open,newData.price,newData.low,newData.high,newData.volume];
      stockData.volumes[len] = {
        value:newData.volume,
        itemStyle: {
          color:color,
          borderColor:borderColor
        }
      };
      stockData.ma5[len] = newData.ma5;
      stockData.ma10[len] = newData.ma10;
      stockData.ma20[len] = newData.ma20;
      stockData.ma60[len] = newData.ma60;
    }
    myChart.setOption({
      xAxis: [
        {
          type: 'category',
          data: stockData.categoryData,
        },
        {
          type: 'category',
          gridIndex: 1,
          data: stockData.categoryData,
        }
      ],
      series: [
        {
          name: 'K线',
          type: 'candlestick',
          data: stockData.values,
          // itemStyle: {
          //   color: upColor,
          //   color0: downColor,
          //   borderColor: bColor,
          //   borderColor0: undefined
          // },
          //https://echarts.apache.org/zh/option.html#series-candlestick
          //https://echarts.apache.org/zh/option.html#series-line.markPoint.data.valueDim
          // markPoint: {
          //   data:[
          //     {
          //       type: 'max',
          //       name: '最大值',
          //       valueDim: 'highest'
          //     },
          //     {
          //       type: 'min',
          //       name: '最小值',
          //       valueDim: 'lowest'
          //     }
          //   ],
          //   symbol: "arrow",
          //   symbolSize: 30,
          //   silent: true,
          //   itemStyle:{
          //     color:"#ecece400"
          //   },
          //   label:{
          //     color:"blue"
          //   }
          // }
        },
        {
          name: 'MA5',
          type: 'line',
          data: stockData.ma5,
          // smooth: true,
          // symbolSize: 0,
          // lineStyle: {
          //   opacity: 0.5
          // },
        },
        {
          name: 'MA10',
          type: 'line',
          data: stockData.ma10,
          // smooth: true,
          // // symbol:'none',
          // symbolSize: 0,
          // lineStyle: {
          //   opacity: 0.5
          // }
        },
        {
          name: 'MA20',
          type: 'line',
          // // symbol:'none',
          // symbolSize: 0,
          // data: stockData.ma20,
          // smooth: true,
          // lineStyle: {
          //   opacity: 0.5
          // }
        },
        {
          name: 'MA60',
          type: 'line',
          // // symbol:'none',
          // symbolSize: 0,
          // data: stockData.ma60,
          // smooth: true,
          // lineStyle: {
          //   opacity: 0.5
          // },
        },
        {
          name: 'Volume',
          type: 'bar',
          // xAxisIndex: 1,
          // yAxisIndex: 1,
          data: stockData.volumes,
        },
      ],
    })
  }
}
function updateLineOption(){
  myChart.setOption({
    graphic: graphicData.value.map(function (item, dataIndex) {
      let startPixel = myChart.convertToPixel({seriesIndex: 0}, item.start);
      let style = item.style;
      if (item.graphic_type==="line"){
        let endPixel = myChart.convertToPixel({seriesIndex: 0}, item.end);
        return {
              id:item.id,
              type: 'line',
              shape: {
                x1: startPixel[0],
                y1: startPixel[1],
                x2: endPixel[0],
                y2: endPixel[1],
                percent: 100
              },
              z:1000,
        }
      }else if (item.graphic_type==="text"){
        let thisLine:Graphic;
        if (item.content==null){
          thisLine = graphicData.value.filter(function (item2) {
            return (item2.group_id===item.group_id&&item2.graphic_type=="line")
          })[0];
        }
        return {
          id:item.id,
          type:"text",
          position: [startPixel[0], startPixel[1]],
          style: {
            fill: style?.color?style?.color:'black',
            // size: style?.size?style?.size:10,
            fontSize:style?.size?style?.size:14,
            lineWidth: style?.line_width?style?.line_width:1,
            text: item.content?item.content:thisLine.start[1].toFixed(3),
            // text: item.content?item.content:item.start[1].toFixed(3),
         },
       }
      }
    })
  })
}
//本来是想仅清除graphic,但是直接全部清除更简单，而且避免了k线图残留闪烁的问题（当切换k线图时，会从上一个直接绘制出新的，现在是由空白绘制新的）
// async function clear_all(){
function clear_all(){
  // groupMap.clear();
  myChart.clear();
  myChart.getZr().clear();
  chartIsInit = false;
  console.log("清除全部数据");
  rawData.value=[];
  graphicData.value.length = 0;
}
async function query_graphic(){
  try {
    const res = await invoke<Graphic[]>('query_graphic_by_code', { code: code }); // 使用 await 等待 invoke 完成
    if (res.length===0){
      return;
    }
    graphicData.value = res;
    handleGraphicData();
  } catch (err) {
    console.log(err);
  }
}
async function query_stocks_day_k_limit(){
  try {
    const data = await invoke<StockData[]>('query_stocks_day_k_limit', { code: code }); // 使用 await 等待 invoke 完成
    if (data[0].date!=nowDate){
      const liveData = await invoke<StockLiveData>('query_live_stock_data_by_code', { code: code }); // 使用 await 等待 invoke 完成
      store.stockinfoG.live_data=liveData;
      const leastData = {
        date:nowDate,
        open:liveData.open,
        close:liveData.price,
        high:liveData.high,
        low:liveData.low,
        vol:liveData.volume,
        ma5:liveData.ma5,
        ma10:liveData.ma10,
        ma20:liveData.ma20,
        ma60:liveData.ma60,
      }
      data.unshift(leastData)
    }
    rawData.value = data.reverse(); // 处理查询到的数据
    myChart.clear();
    myChart.setOption(init_option(),true);
    myChart.resize();
    chartIsInit = true;
    return;
    // myChart.hideLoading();
  } catch (err) {
    console.log(err);
  }
}

function save_graphic(){
  let data = graphicData.value;
  invoke('save_graphic', { code:code,graphic: data }).then(() => {
    emit("graphic_change",{})
  }).catch((err) => {
    console.log("保存图形失败",err);
  });
}
function wheelChangeCode(event: WheelEvent){
  const deltaY = event.deltaY; // 滚动的垂直距离
  debouncedFunction(deltaY)
}
function unlockZoom(event:KeyboardEvent){
  if (event.ctrlKey) {
    if (!isCtrlPressed){
      isCtrlPressed = true;
      // console.log("ctrl被按下,解锁图表")
      setZoomLock(false);
    }
  }
}
//键盘抬起锁上放缩
function lockZoom(event:KeyboardEvent){
  if (event.key === 'Control') {
    if (isCtrlPressed){
      isCtrlPressed = false;
      // console.log("ctrl被抬起,锁定图表")
      setZoomLock(true);
    }
  }
}
//为true的时候无法进行图表缩放，但是可以监听到鼠标滚动事件
function setZoomLock(flag:boolean){
  console.log("设置图表缩放锁定",flag)
  myChart.setOption({
    dataZoom: [
      {
        type: 'inside',
        xAxisIndex: [0, 1],
        zoomLock: flag
      },
    ],
  })
}
async function scrollEvent(deltaY:number){
  if (inputVisible.value){ //当显示输入框时，不响应滚动事件
    return;
  }
  //打印下面的信息
  // console.log("滚动事件",store.stockinfoG?.code,store.stockinfoGs);
  let index = store.stockinfoGs.findIndex((item)=>item.code==store.stockinfoG?.code);
  if (index !== -1) {
    if (deltaY > 0) {
      // 向下滚动
      if (index === store.stockinfoGs.length - 1) {
        // 如果当前是最后一个元素，则回到第一个元素
        successNotification("第一个");
        store.stockinfoG = store.stockinfoGs[0];
      } else {
        // 否则滚动到下一个元素
        store.stockinfoG = store.stockinfoGs[index + 1];
      }
    } else if (deltaY < 0) {
      // 向上滚动
      if (index === 0) {
        // 如果当前是第一个元素，则回到最后一个元素
        store.stockinfoG = store.stockinfoGs[store.stockinfoGs.length - 1];
      } else {
        // 否则滚动到上一个元素
        store.stockinfoG = store.stockinfoGs[index - 1];
      }
    }
    save_graphic();
    code = store.stockinfoG!.code;
    // myChart.showLoading();
    myChart.clear();
    await updateChart();
  } else {
    console.log('stockinfoG不在stockinfoGs数组中',code,store.stockinfoGs);
  }
}

let paintState: PaintState = PaintState.Null;
let currentCount = 0;
let start: number[];
let line: echarts.graphic.Line
function handlePaint(state: PaintState){
  paintState = state;
  currentCount = 0;
  start = [];
  if (state==PaintState.Null){
    myChart.getZr().off('click', clickHandler);
    myChart.getZr().off('mousemove', hoverHandler);
  }else {
    // if (state==PaintState.HLS||state==PaintState.PHLS||state==PaintState.LS){
    //   myChart.getZr().on('mousemove', hoverHandler);
    // }
    myChart.getZr().on('mousemove', hoverHandler);
    myChart.getZr().on('click', clickHandler);
  }
}
const hoverHandler = function (params: any) {
  const pointInPixel = [params.offsetX, params.offsetY];
  let zr = myChart.getZr();
  if (myChart.containPixel('grid',pointInPixel)) {
    if (paintState==PaintState.Text){
      zr.setCursorStyle('text')
    }else {
      zr.setCursorStyle('crosshair')
      if(currentCount==1){
        zr.remove(line);
        if (paintState==PaintState.HL||paintState==PaintState.HLS||paintState==PaintState.PHL||paintState==PaintState.PHLS){
          line = get_line(start,[pointInPixel[0],start[1]])
        }else if (paintState==PaintState.LS){
          line = get_line(start,pointInPixel)
        }
        zr.add(line)
      }
    }
    // zr.remove(line);
    // zr.setCursorStyle('text')
    // if(currentCount==1){
    //   if (paintState==PaintState.HL||paintState==PaintState.HLS||paintState==PaintState.PHL||paintState==PaintState.PHLS){
    //     line = get_line(start,[pointInPixel[0],start[1]])
    //   }else if (paintState==PaintState.LS){
    //     line = get_line(start,pointInPixel)
    //   }
    //   zr.add(line)
    // }
  }
};
const clickHandler = async function (params: any) {
  const pointInPixel = [params.offsetX, params.offsetY];
  if (myChart.containPixel('grid',pointInPixel)) {
    // let clickPointData = myChart.convertFromPixel({seriesIndex: 0}, pointInPixel);
    if (currentCount==0){
      start = pointInPixel;
      if (paintState==PaintState.Text){
        inputVisible.value = true;
        await nextTick();
        await nextTick();
        if(inputRef.value){
          inputRef.value.focus();
        }
      }else {
        currentCount = 1;
      }
    }else if (currentCount == 1){
      if (paintState==PaintState.HL||paintState==PaintState.HLS||paintState==PaintState.PHL||paintState==PaintState.PHLS){
        let startPointData = myChart.convertFromPixel({seriesIndex: 0},[start[0], start[1]]);
        let endPointData = myChart.convertFromPixel({seriesIndex: 0},[pointInPixel[0], start[1]]);
        let groupId = generateId();
        // graphicData.value.push({
        newGraphicData.push({
          group_id: groupId,
          code: code,
          id: generateId(),
          graphic_type: "line",
          start: startPointData,
          end: endPointData,
          horizontal:true
        });
        if(paintState==PaintState.PHLS||paintState==PaintState.PHL){
          //为了让标价线的价格文本稍微在线上一点
          let startPointData = myChart.convertFromPixel({seriesIndex: 0},[start[0], start[1]-15]);
          newGraphicData.push({
            group_id: groupId,
            code: code,
            id: generateId(),
            graphic_type: "text",
            start: [startPointData[0], startPointData[1]],
            end:[0,0],
            style:{
              color:"#da5c0d"
            }
          })
        }
      }else if (paintState==PaintState.LS){
        // let endPoint = myChart.convertFromPixel({seriesIndex: 0},[clickPointData[0], clickPointData[1]]);
        let startPointData = myChart.convertFromPixel({seriesIndex: 0},[start[0], start[1]]);
        let endPointData = myChart.convertFromPixel({seriesIndex: 0},[pointInPixel[0], pointInPixel[1]]);
        // newLineR.value.end = [clickPointData[0],clickPointData[1]];
        // graphicData.value.push({
        newGraphicData.push({
          group_id: generateId(),
          code: code,
          id: generateId(),
          graphic_type: "line",
          start: startPointData,
          end: endPointData
        })
      }
      handleNewGraphicData();
      myChart.getZr().remove(line);
      currentCount = 0;
    }
  }
};
function newText(){
  const text = inputValue.value;
  if (text.length!=0){
    let startPointData = myChart.convertFromPixel({seriesIndex: 0},[start[0], start[1]]);
    let groupId = generateId();
    // graphicData.value.push({
    newGraphicData.push({
      group_id: groupId,
      code: code,
      id: generateId(),
      graphic_type: "text",
      start: startPointData,
      end: [0,0],
      content: text,
      style:{
        color:newTextColor.value,
        size:newTextFontSize.value,
      }
    });
    handleNewGraphicData();
  }
  inputValue.value = "";
  inputVisible.value = false
}
function get_line(start:number[],end:number[]){
  return new echarts.graphic.Line({
    shape:{
      x1: start[0],
      y1: start[1],
      x2: end[0],
      y2: end[1],
      percent: 100
    },
    style:{
      fill:'red',
    },
  })
}
function init_option(){
  const data = handleRawData(rawData.value);
  return {
    animation: false,
    legend: {
      top: 10,
      // left: 'left',
      left: '4%',
      data: ['K线', 'MA5', 'MA10', 'MA20', 'MA60']
      // data: ['K线', ]
    },
    tooltip: {
      // trigger: 'axis',// https://echarts.apache.org/zh/option.html#grid.tooltip.trigger
      trigger: 'item',
      axisPointer: {
        type: 'cross',
        animation:false
        // snap:true 坐标轴指示器是否自动吸附到点上。默认自动判断。好像是默认启用 https://echarts.apache.org/zh/option.html#grid.tooltip.axisPointer.snap
      },
      transitionDuration: 0,
      borderWidth: 1,
      borderColor: '#776d6d',
      backgroundColor : 'rgba(50,50,50,0)',
      padding: 5,
      textStyle: {
        color: '#000'
      },
      formatter: function (params: any) { // params 是 formatter 需要的数据集
        let htmlContent;
        if (params.seriesType == "candlestick") {
          let change = (params.data[2] - params.data[1]).toFixed(3);
          const { openClass, closeClass, changeClass } = formatPriceLabel(params.data[1], params.data[2], change);
          htmlContent = `<div class="column tip">
      <label style="background-color: rgb(241, 241, 148)">${params.name}/${computeWeek(params.name)}</label>
      <hr style="margin:0;"> <!-- 添加水平线并设置上下边距 -->
      <label class="${openClass}">开盘：${params.data[1]}</label>
      <label class="red-label">最高：${params.data[4]}</label>
      <label class="green-label">最低：${params.data[3]}</label>
      <label class="${closeClass}">收盘：${params.data[2]}</label>
      <label class="${changeClass}">涨跌：${change}</label>
      <label class="${changeClass}">涨幅：${params.data[6]}%</label>
<!--      <label>成交量：${params.data[5]}</label>-->
      <label>成交量：${formatLargeNumber(params.data[5])}</label>
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
        backgroundColor: '#e1bf2c',
        color:"black",
        precision:3
      }
    },
    toolbox: {
      show: true,
      feature: {
        myTool3: {
          show: true,
          title: '保存图线',
          // icon: 'image://https://echarts.apache.org/zh/images/favicon.png',
          icon: 'path://M426.666667 682.666667v42.666666h170.666666v-42.666666h-170.666666z m-42.666667-85.333334h298.666667v128h42.666666V418.133333L605.866667 298.666667H298.666667v426.666666h42.666666v-128h42.666667z m260.266667-384L810.666667 379.733333V810.666667H213.333333V213.333333h430.933334zM341.333333 341.333333h85.333334v170.666667H341.333333V341.333333z',
          onclick: function (){
            save_graphic();
          }
        },
        myTool2: {
          show: true,
          title: '画线',
          // icon: 'image://https://echarts.apache.org/zh/images/favicon.png',
          icon: 'path://M811.562667 348.202667l78.848-78.848a10.666667 10.666667 0 0 0 0-15.082667l-120.682667-120.682667a10.666667 10.666667 0 0 0-15.082667 0l-78.848 78.848 135.765334 135.765334z m-45.248 45.248l-135.765334-135.765334L212.053333 676.181333 171.306667 852.693333l176.512-40.725333L766.293333 393.450667zM814.997333 88.32l120.682667 120.682667a74.666667 74.666667 0 0 1 0 105.6L386.56 863.701333a32 32 0 0 1-15.424 8.533334L135.829333 926.570667a32 32 0 0 1-38.4-38.378667l54.314667-235.306667a32 32 0 0 1 8.554667-15.445333L709.397333 88.32a74.666667 74.666667 0 0 1 105.6 0z',
          onclick: function (){
            showPaintTool();
          }
        },
        restore: {},
        saveAsImage: {}
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
      ],
    },
    grid: [
      {
        left: '4%',
        right: '3%',
        height: '65%'
      },
      {
        left: '4%',
        right: '3%',
        top: '75%',
        height: '15%'
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
        start: 80,
        end: 100,
        zoomOnMouseWheel: "ctrl",// 启用鼠标滚轮触发缩放
        zoomLock: true
      },
      {
        show: true,
        xAxisIndex: [0, 1],
        type: 'slider',
        top: '93%',
        start: 85,
        end: 100
      }
    ],
    series: [
      {
        name: 'K线',
        type: 'candlestick',
        data: data.values,
        itemStyle: {
          color: upColor,
          color0: downColor,
          borderColor: bColor,
          borderColor0: undefined
        },
        //https://echarts.apache.org/zh/option.html#series-candlestick
        //https://echarts.apache.org/zh/option.html#series-line.markPoint.data.valueDim
        markPoint: {
          data:[
            {
              type: 'max',
              name: '最大值',
              valueDim: 'highest'
            },
            {
              type: 'min',
              name: '最小值',
              valueDim: 'lowest'
            }
          ],
          symbol: "arrow",
          symbolSize: 30,
          silent: true,
          itemStyle:{
            color:"#ecece400"
          },
          label:{
            color:"blue"
          }
        }
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
        data: data.volumes,
      },
    ],
  }
}
function computeWeek(date:string){
  var date = new Date(date)
  var day = date.getDay();
  // 根据获取的数字，转换为对应的中文星期几
  var weekDays = ["日", "一", "二", "三", "四", "五", "六"];
  var weekDay = weekDays[day];

  // console.log(weekDay); // 输出：周四
  return weekDay
}
function formatPriceLabel(open:number, close:number, change:number) {
  let openClass = 'black-label';
  let closeClass = 'black-label';
  let changeClass = 'black-label';
  if (open < close) {
    openClass = 'green-label';
    closeClass = 'red-label';
  } else if (open > close) {
    openClass = 'red-label';
    closeClass = 'green-label';
  }
  if (change > 0) {
    changeClass = 'red-label'; // 涨幅高于0，红色
  } else if (change < 0) {
    changeClass = 'green-label';   // 涨幅低于0，绿色
  }
  return {
    openClass: openClass,
    closeClass: closeClass,
    changeClass: changeClass
  };
}
function formatLargeNumber(num:number) {
  // 判断是否为数字
  if (isNaN(num)) {
    return '非数字';
  }
  // 转换并格式化数字  423721924
  let formatted;
  if (num >= 1e8) { // 亿
    formatted = (num / 1e8).toFixed(2) + '亿';
  } else if (num >= 1e4) { // 万
    formatted = (num / 1e4).toFixed(2) + '万';
  } else { // 千以下直接显示
    formatted = num.toString();
  }
  return formatted;
}
function getFormattedDate(): string {
  const now = new Date();
  const year = now.getFullYear();
  const month = String(now.getMonth() + 1).padStart(2, '0'); // 月份从0开始，所以需要+1，并用padStart补全为两位数字
  const day = String(now.getDate()).padStart(2, '0'); // 用padStart补全为两位数字
  return `${year}-${month}-${day}`;
}
function calculateChangeRate(openPrice:number, closePrice:number) {
  // 计算涨跌幅
  const changeRate = ((closePrice - openPrice) / openPrice) * 100;
  // 保留两位小数并返回
  return changeRate.toFixed(2);
}
async function showPaintTool(){
  const appWindow = await WebviewWindow.getByLabel('tool')
  await appWindow?.show()
}

// function deleteGraphic(id:string){
//   invoke('delete_graphic_by_id',{id:id}).then(()=>{
//     successNotification("删除成功");
//     //从列表中删除id为id的图形
//     const index = graphicData.value.findIndex((item:any)=>item.id === id);
//     if (index !== -1) {
//       graphicData.value.splice(index, 1);
//     }
//   }).catch((e)=>{
//     errorNotification(e.toString())
//   })
// }
function deleteGroupGraphic(group_id:string){
  invoke('delete_graphic_by_group_id',{groupId:group_id}).then(()=>{
    successNotification("删除成功");
    //从列表中删除所有group_id为group_id的图形
    const deleteGraphics = graphicData.value.filter((item:any)=>item.group_id === group_id);
    myChart.setOption({ //本来想把这个封装到updateLineOption里面的，发现默认值给action是merge还是replace都不行。
      //function updateLineOption(action: any='replace或者merge')无法更新位置
      graphic: deleteGraphics.map(function (item) {
        if (item.graphic_type==="line"){
          return {
            id:item.id,
            type: 'line',
            $action: 'remove',
          }
        }else if(item.graphic_type==="text"){
          return {
            type:"text",
            id: item.id,
            $action: 'remove',
          }
        }
      })
    });
    graphicData.value = graphicData.value.filter((item:any)=>item.group_id !== group_id);
    // if(group_id === ""){
    //   //从列表中删除所有group_id为group_id的图形
    //   graphicData.value = graphicData.value.filter((item:any)=>item.group_id !== group_id);
    // }
  }).catch((e)=>{
    errorNotification(e.toString())
  })
}
</script>

<template>
  <div id="main" ref="chart"  class="candle-chart-container" ></div>
  <context-menu
      v-model:show="contextMenuShow"
      :options="options"
  >
    <context-menu-item label="编辑此图形" @click="" />
    <context-menu-item label="管理所有图形" @click="" />
    <context-menu-sperator />
<!--    <context-menu-item label="删除当前图形" @click="deleteGraphic(options.id)" />-->
    <context-menu-item label="删除当前组全部图形" @click="deleteGroupGraphic(options.group_id)" />
    <!--      <context-menu-group label="Menu with child">-->
    <!--        <context-menu-item label="删除" @click="onMenuClick(2)" />-->
    <!--        <context-menu-item label="Item2" @click="onMenuClick(3)" />-->
    <!--      </context-menu-group>-->
  </context-menu>
  <el-dialog v-model="inputVisible" :show-close="false" draggable="true" width="400" align-center style="padding: 0">
    <template #header="{ }">
      <div class="my-header">
        <label style="font-size: 14px;margin-left: 15px;font-family:sans-serif">添加文本</label>
        <inline-svg src="../assets/svg/close.svg" class="small-close"  @click.left="inputVisible=false"></inline-svg>
      </div>
    </template>
    <div style="margin: 20px">
      <el-input v-model="inputValue" ref="inputRef" placeholder="请输入文字内容"  type="textarea" rows="3" clearable ></el-input>
      <div style="margin-top: 20px">
        <label>颜色</label><el-color-picker v-model="newTextColor" />
        <label style="margin-left: 20px" >大小</label><el-input-number v-model="newTextFontSize" :min="9" :max="25" style="margin-left: 10px;width: 110px"/>
      </div>
    </div>
    <div class="dialog-footer right" style="margin-top: 20px">
      <el-button class="dialog-button" @click="inputVisible=false">取消</el-button>
      <el-button class="dialog-button dialog-confirm" type="primary" @click="newText">
        确定
      </el-button>
    </div>
  </el-dialog>
</template>

<style >
.dialog-button{
  height: 25px;
  width: 35px;
  font-size: 13px;
  background: rgba(229, 219, 219, 0.85);
  color:black;
  font-weight: bold;
  border-color: #9170b000;
}

.candle-chart-container{
  width: 85%;
  height: 100%;
  border: #f25378 1px solid;
}
.tip{
  font-family:"Adobe 黑体 Std R",serif;
  font-size: 13px;
  border-color: #24c8db00;
  border-width: 1px;
  border-style: solid; /* 添加边框样式 */
  width: 110px;
  background-color: #ecece498;
}
.el-color-picker{
  margin-left: 10px;
}
.red-label {
  color: red;
}

.green-label {
  color: green;
}
.black-label {
  color: black;
}
/*.el-dialog__header{
  display: none;
}*/
</style>
