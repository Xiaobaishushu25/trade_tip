<script setup lang="ts">
import {store} from "../../store.ts";
import {useRouter} from "vue-router";
const router = useRouter()

function back(){
  router.back();
}
// const stockInfoG = store.stockinfoG
</script>

<template>
  <div class="detail column">
    <el-button @click="back()">退回</el-button>-->

    <label>{{store.stockinfoG!.code}}</label>
    <label>{{store.stockinfoG!.name}}</label>
  </div>

</template>

<style scoped>

</style>