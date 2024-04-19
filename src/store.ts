// store.js
import { reactive } from 'vue'
import {StockGroup, StockInfo} from "./type.ts";

export const store:{stockInfo:StockInfo[],stockGroups:StockGroup[]} = reactive({
    stockInfo: [],
    stockGroups:[]
})