<script setup lang="ts">
import {onMounted,ref} from "vue";
import {invoke} from "@tauri-apps/api/core";
import {store} from "../../store.ts";

const props = defineProps({
  code: {
    type: String,
    required: true
  },
  name: {
    type: String,
    required: true
  }
})
const groupList = ref<[]>()
const selectGroups = ref<[]>()
const emit = defineEmits([ "hideDialog" ]);
const hideDialog = () => {
  emit('hideDialog');
}
onMounted(() => {
  console.log(props.code)
  groupList.value = store.stockGroups.map((item) => {
    if (item.name!="持有")
      return item.name
    return ""
  }).filter((item) => {
    return item!=""
  })
  groupList.value?.push("ETF")
  console.log(`allgroupList是`+store.stockGroups)
  console.log(`allgroupList是`+store.stockGroups.length)
  console.log(`groupList是`+groupList.value)
  invoke<string[]>("query_groups_by_code", {code: props.code}).then((res) => {
    selectGroups.value = res
    selectGroups.value?.push("全部")
    console.log("selectGroups是"+selectGroups.value)
    console.log("selectGroups是"+selectGroups.value.length)
  }).catch((err) => {
    console.log(err)
  })
})
function newGroup() {
  console.log("newGroup")
}
function addg() {
  groupList.value.push("新分组")
}
</script>

<template>
  选择自选分组
  <div class="column">
    <div class="row" @click.left="newGroup()">
      <inline-svg src="./src/assets/svg/add.svg" class="min-icon"></inline-svg>
      <label style="color: orange;cursor: pointer">新建分组</label>
    </div>
<!--    <el-checkbox-group v-model="selectGroups" class="self-column"  >-->
    <el-checkbox-group v-model="selectGroups" class="column"  >
      <el-checkbox
          v-for="name in groupList"
          :key="name"
          :label="name"
          :value="name"
          :disabled="name==`全部`"
          size="large"
          style="transform: scale(1.15);"
      ></el-checkbox>
<!--      <div v-for="name in groupList" :key="name" class="row">-->
<!--        <el-checkbox-->
<!--            :label="name"-->
<!--            :value="name"-->
<!--            :disabled="name==`全部`"-->
<!--            size="large"-->
<!--        ></el-checkbox>-->
<!--      </div>-->
    </el-checkbox-group>
  </div>
  <div class="dialog-footer">
    <el-button class="button" @click="hideDialog">取消</el-button>
    <el-button class="button confirm" type="primary" @click="hideDialog">
      确定
    </el-button>
  </div>
  {{groupList}}
  {{selectGroups}}
  <button @click="addg()">你好</button>

</template>
<style>
.el-checkbox__input.is-checked .el-checkbox__inner{
  background: orange;
  border-color: orange;
}
.el-checkbox__input .el-checkbox__inner:hover{
  border-color: orange;
}
.el-checkbox__input.is-checked+.el-checkbox__label{
  color: black;
}
.el-checkbox__inner{
  transition: none;
}
.el-checkbox{
  height: 20px;
}
.el-checkbox.el-checkbox--large{
  height: 25px;
}

</style>

<style scoped>
.row{
  gap: 10px;
  cursor: pointer;
  align-items: center;
}
.button{
  height: 25px;
  width: 35px;
  font-size: 13px;
  background: rgba(229, 219, 219, 0.85);
  color:black;
  font-weight: bold;
  border-color: #9170b000;
}
.confirm{
  color: #9d6a09;
}
</style>