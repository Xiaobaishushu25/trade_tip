<script setup lang="ts">

import Header from "./Header.vue";
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