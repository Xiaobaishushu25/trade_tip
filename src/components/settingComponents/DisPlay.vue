<script setup lang="ts">
import {onMounted,ref,watch} from "vue";
const aExtend = ref(true) //always extend
</script>

<template>
  <div >
    <el-scrollbar max-height="calc(100vh - 60px)">
      <label class="title">蜡烛图</label>
      <el-card shadow="always" style="margin: 10px;background-color:#2f2f2f40;border-radius: 10px;padding: 5px">
        <div class="setting-row-container">
          <label class="label-text">总是将标价线段延伸到图表的右边界:</label>
          <el-switch
              v-model="aExtend"
              class="ml-2"
              style="--el-switch-on-color: #13ce66;"
          />
        </div>
        <el-divider style="margin: 5px" />
        <label class="label-text">总是将标价线段延伸到图表的右边界:</label>
        <el-switch
            v-model="aExtend"
            class="ml-2"
            style="--el-switch-on-color: #13ce66;"
        />
      </el-card>
      <label class="title">蜡烛图</label>
      <el-card shadow="always" style="margin: 10px;background-color:#2f2f2f40;border-radius: 10px;padding: 5px">
        <label class="label-text">总是将标价线段延伸到图表的右边界:</label>
        <el-switch
            v-model="aExtend"
            class="ml-2"
            style="--el-switch-on-color: #13ce66;"
        />
        <el-divider style="color: red;margin: 5px" />
        <label class="label-text">总是将标价线段延伸到图表的右边界:</label>
        <el-switch
            v-model="aExtend"
            class="ml-2"
            style="--el-switch-on-color: #13ce66;"
        />
      </el-card>
      <label class="title">蜡烛图</label>
      <el-card shadow="always" style="margin: 10px;background-color:#2f2f2f40;border-radius: 10px;padding: 5px">
        <label class="label-text">总是将标价线段延伸到图表的右边界:</label>
        <el-switch
            v-model="aExtend"
            class="ml-2"
            style="--el-switch-on-color: #13ce66;"
        />
        <el-divider style="color: red;margin: 5px" />
        <label class="label-text">总是将标价线段延伸到图表的右边界:</label>
        <el-switch
            v-model="aExtend"
            class="ml-2"
            style="--el-switch-on-color: #13ce66;"
        />
      </el-card>
      <label class="title">蜡烛图</label>
      <el-card shadow="always" style="margin: 10px;background-color:#2f2f2f40;border-radius: 10px;padding: 5px">
        <label class="label-text">总是将标价线段延伸到图表的右边界:</label>
        <el-switch
            v-model="aExtend"
            class="ml-2"
            style="--el-switch-on-color: #13ce66;"
        />
        <el-divider style="color: red;margin: 5px" />
        <label class="label-text">总是将标价线段延伸到图表的右边界:</label>
        <el-switch
            v-model="aExtend"
            class="ml-2"
            style="--el-switch-on-color: #13ce66;"
        />
      </el-card>
      <label class="title">蜡烛图</label>
      <el-card shadow="always" style="margin: 10px;background-color:#2f2f2f40;border-radius: 10px;padding: 5px">
        <label class="label-text">总是将标价线段延伸到图表的右边界:</label>
        <el-switch
            v-model="aExtend"
            class="ml-2"
            style="--el-switch-on-color: #13ce66;"
        />
        <el-divider style="color: red;margin: 5px" />
        <label class="label-text">总是将标价线段延伸到图表的右边界:</label>
        <el-switch
            v-model="aExtend"
            class="ml-2"
            style="--el-switch-on-color: #13ce66;"
        />
      </el-card>
      <label class="title">蜡烛图</label>
      <el-card shadow="always" style="margin: 10px;background-color:#2f2f2f40;border-radius: 10px;padding: 5px">
        <label class="label-text">总是将标价线段延伸到图表的右边界:</label>
        <el-switch
            v-model="aExtend"
            class="ml-2"
            style="--el-switch-on-color: #13ce66;"
        />
        <el-divider style="color: red;margin: 5px" />
        <label class="label-text">总是将标价线段延伸到图表的右边界:</label>
        <el-switch
            v-model="aExtend"
            class="ml-2"
            style="--el-switch-on-color: #13ce66;"
        />
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