<script setup lang="ts">
import {onMounted,ref,watch} from "vue";
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
  <div >
    <el-scrollbar max-height="calc(100vh - 60px)">
      <label class="title">证券表</label>
      <el-card shadow="always" style="margin: 10px;background-color:#2f2f2f40;border-radius: 10px;padding: 5px">
        <div class="setting-row-container">
          <label class="label-text">导出证券数据：</label>
          <el-input-number v-model="updateFreq" :min="15" :max="120" :step="5" @change="handleUpdateFreq" />
        </div>
        <el-divider style="margin: 5px" />
        <div class="setting-row-container">
          <label class="label-text">箱体分区数量：</label>
          <el-input-number v-model="boxNum" :min="3" :max="8" @change="handleBoxNum" />
        </div>
      </el-card>
      <label class="title">蜡烛图</label>
      <el-card shadow="always" style="margin: 10px;background-color:#2f2f2f40;border-radius: 10px;padding: 5px">
        <div class="setting-row-container">
          <label class="label-text">股票数据更新频率(s)：</label>
          <el-input-number v-model="updateFreq" :min="15" :max="120" :step="5" @change="handleUpdateFreq" />
        </div>
        <el-divider style="margin: 5px" />
        <div class="setting-row-container">
          <label class="label-text">箱体分区数量：</label>
          <el-input-number v-model="boxNum" :min="3" :max="8" @change="handleBoxNum" />
        </div>
      </el-card>

    </el-scrollbar>
  </div>
</template>
<style>
.el-card__body{
  padding: 5px;
}
</style>
<style scoped>
.setting-row-container {
  display: flex;
  justify-content: space-between;
  align-items: center; /* 保证垂直居中 */
  width: 100%; /* 确保容器占据整行 */
}
.title{
  font-size: 20px;
  margin-left:20px;
  font-family: "Arial Black";
  font-weight: 600;
}
.label-text{
  font-size: 15px;
}
</style>