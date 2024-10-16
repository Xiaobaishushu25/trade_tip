<script setup lang="ts">
const updateFreq = ref(5)
const boxNum = ref(3)
const handleBoxNum = (value: number) => {
  console.log(value)
}
const handleUpdateFreq = (value: number) => {
  console.log(value)
}
</script>

<template>
  <el-scrollbar height="400px">
    数据
    <el-card style="width: 480px" shadow="always">
      股票数据更新频率(s)： <el-input-number v-model="updateFreq" :min="15" :max="120" :step="5" @change="handleUpdateFreq" />
      <el-divider />
      箱体分区：

    </el-card>
    蜡烛图
    <el-card style="width: 480px" shadow="always">
      箱体分区： <el-input-number v-model="boxNum" :min="3" :max="8" @change="handleBoxNum" />
      <el-divider />
      箱体分区：

    </el-card>

  </el-scrollbar>
</template>

<style scoped>

</style>