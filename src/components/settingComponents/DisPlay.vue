<script setup lang="ts">
import {onMounted,ref,watch} from "vue";
import {store} from "../../store.ts";
//todo 由于设置窗口是一个单独的webview，读不到另一个webview的store，所以需要通过emit等来获取store
const aExtend = ref<boolean>(false);
const num = ref<number>(10);
function handleUpdateBsSize(){}
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
        <div class="setting-row-container">
          <label class="label-text">B/S点大小:</label>
<!--          <el-input-number v-model="store.config.display_config.bs_size" :min="5" :max="30" :step="1" class="custom-input" @change="handleUpdateBsSize" />-->
          <el-input-number v-model="num" :min="5" :max="30" :step="1" class="custom-input" @change="handleUpdateBsSize" />
        </div>
        <el-divider style="margin: 5px" />
        <div class="setting-row-container">
          <label class="label-text">默认从?开始显示K线(%):</label>
          <el-input-number v-model="num" :min="0" :max="99" :step="1" class="custom-input" @change="handleUpdateBsSize" />
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
.setting-row-container {
  display: flex;
  justify-content: space-between;
  align-items: center; /* 保证垂直居中 */
  width: 100%; /* 确保容器占据整行 */
  height: 30px;
}
.el-card__body{
  padding: 5px;
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
</style>