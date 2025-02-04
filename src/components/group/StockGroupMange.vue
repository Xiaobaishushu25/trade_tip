<script setup lang="ts">
import StockGroupSelect from "./StockGroupSelect.vue";
import {ref,watch} from "vue";
import InlineSvg from "vue-inline-svg";
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

<!--  <el-dialog v-model="dialogFormVisible" :title="`管理 ${name}`" width="320px">-->
<!--    <StockGroupSelect :code="code" :name="name" @hideDialog="hideDialog"></StockGroupSelect>-->
<!--  </el-dialog>-->

  <el-dialog v-model="dialogFormVisible" :show-close="false" :draggable=true width="320" align-center style="padding: 0">
    <template #header="{ }">
      <div class="my-header">
        <div>
          <label style="font-size: 14px;margin-left: 15px;font-family:sans-serif">管理{{name}}</label>
          <label style="font-size: 12px;color:red;font-family:sans-serif">(不勾选全部即彻底删除)</label>
        </div>
<!--        <inline-svg src="./src/assets/svg/close.svg" class="small-close"  @click.left="dialogFormVisible=false"></inline-svg>-->
        <inline-svg src="../assets/svg/close.svg" class="small-close"  @click.left="dialogFormVisible=false"></inline-svg>
      </div>
    </template>
    <StockGroupSelect :code="code" :name="name" @hideDialog="hideDialog"></StockGroupSelect>
  </el-dialog>
</template>

<style scoped>

</style>