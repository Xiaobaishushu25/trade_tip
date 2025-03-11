<script setup lang="ts">

import Header from "./Header.vue";
import {listen} from "@tauri-apps/api/event";
import {errorNotification} from "../utils.ts";
onMounted(async ()=>{
  await listen("get_live_data_error", (msg)=>{
    errorNotification( `获取实时数据失败:${msg.payload}`);
  });
});
</script>

<template>
  <Header></Header>
  <div class="main-container">
    <router-view v-slot="{ Component }">
      <keep-alive>
        <component :is="Component" />
      </keep-alive>
    </router-view>
  </div>
</template>

<style scoped>
.main-container{
  min-height: calc(100vh - 30px);
  height: calc(100vh - 30px);
  width: 100vw;
}
</style>