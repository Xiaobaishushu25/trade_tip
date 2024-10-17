<script setup lang="ts">
import { open } from '@tauri-apps/plugin-dialog';
const emit = defineEmits([ "clearFilter" ]);
async function importData() {
  const file = await open({
    title: '选择交易数据',
    multiple: false,
    directory: false,
    filters: [
      {
        name: 'csv',
        extensions: ['csv'],
      },
    ],
  });
  console.log(file);
// Prints file path or URI
}
</script>

<template>
<div class="row">
  <el-button plain @click.left="importData">开启筛选</el-button>
  <el-button plain @click.left="emit('clearFilter')">取消筛选</el-button>
  <el-button plain @click.left="importData">导入数据</el-button>
</div>
</template>
<style>
.el-button{
  padding: 5px;
}
.el-button.is-plain{
  --el-button-text-color: black;
  --el-button-hover-text-color: black;
  --el-button-hover-bg-color: #83808050;
  --el-button-border-color: #838080;
  --el-button-hover-border-color: #838080;
}
</style>
<style scoped>

</style>