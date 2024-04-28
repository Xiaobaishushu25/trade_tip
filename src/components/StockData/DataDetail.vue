<script setup lang="ts">
import {StockInfoG} from "../../type.ts";
import {store} from "../../store.ts";
import CandleChartNewNew from "../CandleChartNewNew.vue";
import Detail from "./Detail.vue";
import NewNewCandleChart from "../NewNewCandleChart.vue";

// interface Props {
//   stock:StockInfoG,
//   stocks:StockInfoG[]
// }
// const props = defineProps<Props>()

</script>

<template>
  <div class="detail-container">
<!--    <CandleChartNewNew></CandleChartNewNew>-->
    <NewNewCandleChart></NewNewCandleChart>
    <Detail></Detail>
  </div>
</template>

<style scoped>
.detail-container{
  display: flex;
  flex-direction: row;
  height: 100%;
  border: #24c8db 1px solid;
}
</style>