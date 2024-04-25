<script setup lang="ts">
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
import {onMounted, ref} from "vue";
// import axios from "axios";
import Header from "./components/Header.vue";
import {StockInfo} from "./type.ts";
import {invoke} from "@tauri-apps/api/core";
import {store} from "./store.ts"
import {useRouter} from "vue-router";
// const stockInfo = ref<StockInfo[]>([]);
onMounted(()=>{
  //https://router.vuejs.org/zh/guide/advanced/composition-api.html#%E5%9C%A8-setup-%E4%B8%AD%E8%AE%BF%E9%97%AE%E8%B7%AF%E7%94%B1%E5%92%8C%E5%BD%93%E5%89%8D%E8%B7%AF%E7%94%B1
  window.addEventListener("contextmenu",  (e) => {e.preventDefault()},false)
  // router.push("/tab")
  // invoke<StockInfo[]>("query_stock_info", {}).then(res => {
  //   console.log(res);
  //   store.stockInfo = res;
  //   store.stockInfo[0].price= 1.56
  //   console.log(store.stockInfo[0].price)
  //   // stockInfo.value = res;
  // }).catch(err => {
  //   console.log(err);
  // })
})
</script>

<template>
<!--  <n-layout>-->
<!--    <n-layout-header >-->
<!--      <Header></Header>-->
<!--    </n-layout-header>-->
<!--    <n-layout-content>-->
<!--      <router-view v-slot="{ Component }">-->
<!--        <keep-alive>-->
<!--          <component :is="Component" />-->
<!--        </keep-alive>-->
<!--      </router-view>-->
<!--    </n-layout-content>-->
<!--  </n-layout>-->
<!--  <Header></Header>-->
<!--  <div class="main-container">-->
<!--    <router-view v-slot="{ Component }">-->
<!--      <keep-alive>-->
<!--        <component :is="Component" />-->
<!--      </keep-alive>-->
<!--    </router-view>-->
<!--  </div>-->
      <router-view v-slot="{ Component }">
        <keep-alive>
          <component :is="Component" />
        </keep-alive>
      </router-view>
</template>

<style scoped>
.main-container{
  min-height: calc(100vh - 30px);
  height: calc(100vh - 30px);
  width: 100vw;
}

</style>
