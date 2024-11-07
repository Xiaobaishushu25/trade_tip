<script setup lang="ts">
import {ref, watch} from "vue";
import {invoke} from "@tauri-apps/api/core";
import {errorNotification, successNotification} from "../../utils.ts";

const date = ref(new Date());
const position = ref();
async function handleChange() {
  const datestr = date.value.toLocaleDateString('zh-CN', { year: 'numeric', month: '2-digit', day: '2-digit' }).replace(/\//g, '-');
  console.log(date, position.value);
  invoke("insert_position", {date: datestr, positionNum: position.value}).then(_ => {
    successNotification("仓位插入成功")
  }).catch(err => {
    errorNotification(`仓位插入失败: ${err}`)
  })  ;
}
</script>

<template>
  <div class="row">
    <el-date-picker
        v-model="date"
        type="date"
        placeholder="选择日期"
    />
    <el-input-number v-model="position" :min="0" :max="100" :precision="2" :controls="false" style="width: 120px;"  @change="handleChange" placeholder="请输入仓位%" />
  </div>
</template>

<style scoped>

</style>