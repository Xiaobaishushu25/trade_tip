// store.js
import { reactive } from 'vue'
import { StockGroup, StockInfo, StockInfoG} from "./type.ts";

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
} = reactive({
    activeGroup:"",
    stockGroups:[],
    stockinfoG:undefined,
    stockinfoGs:[],
    boxData:{},
    // count:"",
})