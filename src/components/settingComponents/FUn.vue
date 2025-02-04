<script setup lang="ts">
import {onBeforeMount, onMounted, ref, watch} from "vue";
import {DataConfig} from "../../type.ts";
import {emit} from "@tauri-apps/api/event";
import {invoke} from "@tauri-apps/api/core";
import {errorNotification, successNotification} from "../../utils.ts";
const updateFreq = ref(5)
const boxNum = ref(3)

const dataConfig = ref<DataConfig>();
onBeforeMount(()=>{ //注意onMounted在组件挂载后（即template已经渲染）执行，所以这里要用onBeforeMount
  const storedObjectString = localStorage.getItem('config');
  const myObjectFromStorage = JSON.parse(storedObjectString);
  dataConfig.value = myObjectFromStorage.data_config;
})
async function handleUpdateDataConfig(){
  //虽然这个配置在前端界面用不上，但是还是与Display设置统一逻辑，全部发送出去用来修改全局存储中的配置，这样保存时直接整个覆盖掉就行。
  await emit("data_config_update", dataConfig.value);
  await invoke("update_data_config", {dataConfig: dataConfig.value}).then((_) => {
    successNotification("更新数据配置成功");
  }).catch((error) => {
    console.log(error);
    errorNotification(`更新数据配置失败：${error}`);
  });
}
const handleBoxNum = (value: number) => {
  console.log(value)
}
const handleUpdateFreq = (value: number) => {
  console.log(value)
}
</script>

<template>
  <div class="setting-container">
    <el-scrollbar max-height="calc(100vh - 60px)">
      <label class="title">证券表</label>
      <el-card shadow="always" style="margin: 10px;background-color:#2f2f2f40;border-radius: 10px;padding: 5px">
        <div class="setting-row-container">
          <label class="label-text">导出证券数据：</label>
          <el-input-number v-model="updateFreq" :min="15" :max="120" :step="5" class="custom-input" @change="handleUpdateFreq" />
        </div>
        <el-divider style="margin: 5px" />
        <div class="setting-row-container">
          <div class="row-no-padding">
            <label class="label-text">箱体分区数量</label>
            <el-tooltip content="将一个箱体均等划为若干区间，其中最上面的区间是上轨区，最下面的区间是下轨区。如箱体分区数量为5，则上和下轨区间各为一份，中间三份为中轨区间。划分越精细，则上下轨区越小。">
              <inline-svg src="../assets/svg/what.svg" class="what"></inline-svg>
            </el-tooltip>
            <label class="label-text">:</label>
          </div>
          <el-input-number v-model="dataConfig.box_num" :min="3" :max="15" class="custom-input" @change="handleUpdateDataConfig" />
        </div>
        <el-divider style="margin: 5px" />
        <div class="setting-row-container">
          <div class="row-no-padding">
            <label class="label-text">日内做T上涨阈值</label>
            <el-tooltip content="如果当日9:00-10:00价格涨幅超过此值，则初步认为当日可以做t。">
              <inline-svg src="../assets/svg/what.svg" class="what"></inline-svg>
            </el-tooltip>
            <label class="label-text">:</label>
          </div>
          <el-input-number v-model="dataConfig.up_t_trigger" :min="0.1" :max="20" :step="0.1" class="custom-input" style="width: 120px" @change="handleUpdateDataConfig" />
        </div>
        <div class="setting-row-container">
          <div class="row-no-padding">
            <label class="label-text">日内做T下跌阈值</label>
            <el-tooltip content="如果当日9:00-10:00价格跌幅超过此值，则初步认为当日可以做t。">
              <inline-svg src="../assets/svg/what.svg" class="what"></inline-svg>
            </el-tooltip>
            <label class="label-text">:</label>
          </div>
          <el-input-number v-model="dataConfig.down_t_trigger" :min="-20" :max="-0.1" :step="0.1" class="custom-input" style="width: 120px" @change="handleUpdateDataConfig" />
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
      <label class="title">数据源</label>
      <el-card shadow="always" style="margin: 10px;background-color:#2f2f2f40;border-radius: 10px;padding: 5px">
        <div class="setting-row-container">
          <label class="label-text">使用AKShare：</label>
          <el-checkbox v-model="dataConfig.use_ak_share" @change="handleUpdateDataConfig"></el-checkbox>
        </div>
        <div class="setting-row-container">
          <label class="label-text">启动命令：</label>
          <el-input v-model="dataConfig.data_server" class="custom-input" :disabled="!dataConfig.use_ak_share" style="width: 700px" @change="handleUpdateDataConfig"/>
        </div>
        <el-divider style="margin: 5px" />
      </el-card>
    </el-scrollbar>
  </div>
</template>
<style>
.el-card__body{
  padding: 5px;
}
.setting-container .el-input{
  --el-input-text-color: black;
  --el-input-focus-border-color: #d5343400;
}
.setting-container .el-input__wrapper{
  box-shadow:none;
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