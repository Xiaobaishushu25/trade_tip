<script setup lang="ts">
const aExtend = ref(true) //alaways extend
</script>

<template>
  <el-scrollbar height="400px">
    蜡烛图
    <el-card style="width: 480px" shadow="always">
      总是将标价线段延伸到图表的右边界：
      <el-switch
          v-model="aExtend"
          class="ml-2"
          style="--el-switch-on-color: #13ce66;"
      />
      <el-divider />
    </el-card>

  </el-scrollbar>
</template>

<style scoped>

</style>