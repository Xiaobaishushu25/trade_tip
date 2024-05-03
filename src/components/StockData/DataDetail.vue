<script setup lang="ts">
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
    <NewNewCandleChart class="chart-container"></NewNewCandleChart>
    <Detail class="info-container"></Detail>
  </div>
</template>

<style scoped>
.detail-container{
  display: flex;
  flex-direction: row;
  height: 100%;
  width: 100%;
  border: #24c8db 1px solid;
}
.chart-container{
  flex: 4; /* 宽度上占据4份 */
}
.info-container{
  flex: 1;
  padding: 1em;
}
</style>