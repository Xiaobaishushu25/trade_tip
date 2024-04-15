<script setup lang="ts">
import {onMounted} from "vue";
import {StockInfo} from "../type.ts";
import {invoke} from "@tauri-apps/api/core";

const stockInfo = ref<StockInfo[]>([]);
onMounted(() => {
  invoke<StockInfo[]>("query_stock_info", {}).then(res => {
    console.log(res);
    stockInfo.value = res;
  }).catch(err => {
    console.log(err);
  })
})
</script>

<template>
</template>

<style scoped>

</style>