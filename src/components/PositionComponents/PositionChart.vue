<script setup lang="ts">
import {listen} from "@tauri-apps/api/event";
import {onMounted} from "vue";
onMounted(async () => {
  let unlistenPositionUpdate = await listen("position_change", ({payload }) => {
    console.log(payload)
  })
})
</script>

<template>

</template>

<style scoped>

</style>