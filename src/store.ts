// store.js
import { reactive } from 'vue'
import {rowData, StockGroup, StockInfoG} from "./type.ts";

// export const store:{stockInfo:StockInfo[],
//     stockGroups:StockGroup[],
//     stockinfoG:StockInfoG|undefined,
//     stockinfoGs:StockInfoG[],
//     boxData:Record<string, number[]>,
// } = reactive({
//     stockInfo: [],
//     stockGroups:[],
//     stockinfoG:undefined,
//     stockinfoGs:[],
//     boxData:{},
//     // count:"",
// })
export const store:{
    activeGroup:string,
    stockGroups:StockGroup[],
    stockinfoG:StockInfoG|undefined,
    stockinfoGs:StockInfoG[],
    boxData:Record<string, number[]>,
    isBlur:boolean,
    rowData:Map<string,rowData>
} = reactive({
    activeGroup:"",//当前选择的分组
    stockGroups:[],//当前所有的分组
    stockinfoG:undefined,//当前选择的股票
    stockinfoGs:[],//当前分组的所有的股票
    boxData:{},//全部的箱线图数据
    isBlur:false,//窗口是否失去焦点
    rowData:new Map()
    // count:"",
})