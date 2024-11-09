<script setup lang="ts">
import {onMounted, onBeforeMount,ref,watch} from "vue";
import {DisplayConfig} from "../../type.ts";
import {emit} from "@tauri-apps/api/event";
//todo 由于设置窗口是一个单独的webview，读不到另一个webview的store，所以需要通过别的方式获取配置

const disConfig = ref<DisplayConfig>();
const aExtend = ref<boolean>(false);
onBeforeMount(()=>{ //注意onMounted在组件挂载后（即template已经渲染）执行，所以这里要用onBeforeMount
  const storedObjectString = localStorage.getItem('config');
  const myObjectFromStorage = JSON.parse(storedObjectString);
  disConfig.value = myObjectFromStorage.display_config;
})
async function handleUpdateDisConfig(){
  await emit("disPlay_update",disConfig.value)
}
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
<!--          <el-input-number v-model="disConfig.bs_size" :min="5" :max="30" :step="1" class="custom-input" @change="handleUpdateDisConfig" />-->
          <el-slider v-model="disConfig.bs_size" :min="5" :max="30" style="width: 30%" @change="handleUpdateDisConfig"/>
        </div>
        <el-divider style="margin: 5px" />
        <div class="setting-row-container">
          <div class="row-no-padding">
            <label class="label-text">默认从?开始显示K线(%)</label>
            <el-tooltip content="设置显示K线范围的起始百分比，例如设置为50，则表示显示50%到100%的K线">
              <inline-svg src="../assets/svg/what.svg" class="what"></inline-svg>
            </el-tooltip>
            <label class="label-text">:</label>
          </div>
          <el-slider v-model="disConfig.k_show_begin" :min="0" :max="95" style="width: 30%" @change="handleUpdateDisConfig"/>
<!--          <el-input-number v-model="num" :min="0" :max="99" :step="1" class="custom-input" @change="handleUpdateDisConfig" />-->
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
.what{
  width: 15px;
  height: 15px;
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