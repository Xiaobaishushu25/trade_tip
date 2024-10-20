<script setup lang="ts">
import { open } from '@tauri-apps/plugin-dialog';
import {invoke} from "@tauri-apps/api/core";
import {errorNotification,successNotification} from "../../utils.ts";
import { ref } from 'vue';


const emit = defineEmits([ "deleteAllRecords","exportRecords" ]);

const dialogVisible = ref(false);
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
  if (file!== null) {
    invoke('read_save_transaction_records', {path: file}).then(data => {
      console.log(data);
      emit('exportRecords',data);
    }).catch(e => {
      console.log(e);
      errorNotification(`导入失败：${e}`)
    });
  }
}
async function exportData() {
  const dir = await open({
    title: '保存交易数据',
    directory: true,
    multiple: false,
  })
  if (dir !== null) {
    const now = new Date();
    const month = String(now.getMonth() + 1).padStart(2, '0'); // 当前月份
    const day = String(now.getDate()).padStart(2, '0'); // 当前日期
    const hours = String(now.getHours()).padStart(2, '0'); // 当前小时
    const minutes = String(now.getMinutes()).padStart(2, '0'); // 当前分钟

    const path = `${dir}/TradeTip_${month}${day}${hours}${minutes}.csv`;
    invoke('save_transaction_records', {path: path}).then(_ => {
      successNotification(`成功导出至`+path);
    }).catch(e => {
      errorNotification(`导出失败：${e}`)
    })
  }
}
async function deleteAll() {
  invoke('delete_all_transaction_records').then(() => {
    emit('deleteAllRecords');
  }).catch(e => {
    errorNotification(`删除全部记录失败：${e}`);
  });
  dialogVisible.value = false;
}
</script>

<template>
<div class="row">
<!--  <el-button plain @click.left="emit('clearFilter')">取消筛选</el-button>-->
  <el-button plain @click.left="exportData">导出全部数据</el-button>
  <el-button plain @click.left="importData">导入数据</el-button>
  <el-button plain @click.left="dialogVisible = true">全部删除</el-button>
  <el-dialog
      v-model="dialogVisible"
      title="删除"
      width="500"
  >
    <span>确认要删除全部交易记录数据吗？(建议先导出备份)</span>
    <template #footer>
      <div class="dialog-footer">
        <el-button plain @click="dialogVisible = false">取消</el-button>
        <el-button plain @click="deleteAll">
          删除
        </el-button>
      </div>
    </template>
  </el-dialog>
</div>
</template>
<style>
.el-button{
  padding: 5px;
  --el-button-active-border-color: #83808000;
  --el-button-outline-color: #83808000;
  --el-button-active-color: #83808000;
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