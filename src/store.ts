// store.js
import { reactive } from 'vue'
import {StockInfo} from "./type.ts";

export const store:{stockInfo:StockInfo[]} = reactive({
    stockInfo: []
})