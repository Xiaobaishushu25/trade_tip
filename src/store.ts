// store.js
import { reactive } from 'vue'
import {PaintState, StockGroup, StockInfo, StockInfoG} from "./type.ts";

export const store:{stockInfo:StockInfo[],
    stockGroups:StockGroup[],
    stockinfoG:StockInfoG|undefined,
    stockinfoGs:StockInfoG[],
} = reactive({
    stockInfo: [],
    stockGroups:[],
    stockinfoG:undefined,
    stockinfoGs:[],
    // count:"",
})