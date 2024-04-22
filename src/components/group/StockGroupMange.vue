<script setup lang="ts">
import StockGroupSelect from "./StockGroupSelect.vue";
import {ref,watch} from "vue";
const props = defineProps({
  showDialog: {
    type: Boolean,
    required: true
  },
  code: {
    type: String,
    required: true
  },
  name: {
    type: String,
    required: true
  }
})
const dialogFormVisible = ref(false)
const emit = defineEmits([ "hideDialog" ]);
const hideDialog = (ok: boolean) => {
  dialogFormVisible.value = false
  emit('hideDialog',ok);
};
//showDialog值变化时(不需要判断布尔值，只要改变就显示)，显示对话框
watch(() => props.showDialog, (_) => {
  dialogFormVisible.value = true
})
</script>

<template>
<!--  <n-modal v-model:show="dialogFormVisible" :title="`管理 ${name}`" style="width: 320px" transform-origin="center" preset="dialog" :show-icon="false">-->
<!--    <n-flex justify="center">-->
<!--      <GroupSelectNew :code="code" :name="name" @hideDialog="hideDialog"></GroupSelectNew>-->
<!--    </n-flex>-->
<!--  </n-modal>-->
  <el-dialog v-model="dialogFormVisible" :title="`管理 ${name}`" width="320px">
<!--    <GroupSelectNew :code="code" :name="name" @hideDialog="hideDialog"></GroupSelectNew>-->
    <StockGroupSelect :code="code" :name="name" @hideDialog="hideDialog"></StockGroupSelect>
  </el-dialog>
</template>

<style scoped>

</style>