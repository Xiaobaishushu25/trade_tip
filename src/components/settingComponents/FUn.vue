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
          <el-input-number v-model="updateFreq" :min="15" :max="120" :step="5" class="custom-input" @change="handleUpdateFreq" />
        </div>
        <el-divider style="margin: 5px" />
        <div class="setting-row-container">
          <label class="label-text">箱体分区数量：</label>
          <el-input-number v-model="boxNum" :min="3" :max="8" class="custom-input" @change="handleBoxNum" />
        </div>
      </el-card>
      <label class="title">蜡烛图</label>
      <el-card shadow="always" style="margin: 10px;background-color:#2f2f2f40;border-radius: 10px;padding: 5px">
        <div class="setting-row-container">
          <label class="label-text">股票数据更新频率(s)：</label>
          <el-input-number v-model="updateFreq" :min="15" :max="120" :step="5" class="custom-input" @change="handleUpdateFreq" />
        </div>
        <el-divider style="margin: 5px" />
        <div class="setting-row-container">
          <label class="label-text">箱体分区数量：</label>
          <el-input-number v-model="boxNum" :min="3" :max="8" class="custom-input" @change="handleBoxNum" />
        </div>
      </el-card>

    </el-scrollbar>
  </div>
</template>
<style>
.el-card__body{
  padding: 5px;
}
.el-input{
  --el-input-text-color: black;
  --el-input-focus-border-color: #d5343400;
}
.el-input__wrapper{
  box-shadow:none;
}
</style>
<style scoped>
.title{
  font-size: 20px;
  margin-left:20px;
  font-family: "Arial Black";
  font-weight: 600;
}
.label-text{
  font-size: 15px;
}
.custom-input {
  background-color: #c8656500; /* 设置背景色 */
  width: 100px; /* 设置宽度 */
  height: 25px; /* 设置高度 */
}
.custom-input:hover{
  border: #cf0f59;
}

</style>