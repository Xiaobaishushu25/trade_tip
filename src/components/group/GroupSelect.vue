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
onMounted(() => {
  console.log(props.code)
  groupList.value = store.stockGroups.map((item) => {
    if (item.name!="持有")
      return item.name
    return ""
  }).filter((item) => {
    return item!=""
  })
  console.log(`allgroupList是`+store.stockGroups)
  console.log(`allgroupList是`+store.stockGroups.length)
  console.log(`groupList是`+groupList.value)
  invoke<string[]>("query_groups_by_code", {code: props.code}).then((res) => {
    selectGroups.value = selectGroups
    console.log("selectGroups是"+selectGroups.value)
    console.log("selectGroups是"+selectGroups.value.length)
  }).catch((err) => {
    console.log(err)
  })
})
function newGroup() {
  console.log("newGroup")
}
</script>

<template>
  选择自选分组
  <div class="column">
    <div class="row" @click.left="newGroup()">
      <inline-svg src="./src/assets/svg/add.svg" class="min-icon"></inline-svg>
      <label style="color: orange;cursor: pointer">新建分组</label>
    </div>
    <el-checkbox-group v-model="groupList" style="background-color: darkred;justify-content: center;text-align: center;" >
      <el-checkbox
          v-for="name in groupList"
          :key="name"
          :label="name"
          :value="name"
          :disabled="name==`全部`"
          size="large"
          style="transform: scale(1.15);"
      ></el-checkbox>
    </el-checkbox-group>
  </div>

</template>
<style>

</style>

<style scoped>
.row{
  gap: 10px;
  cursor: pointer;
}
.el-checkbox {
  width: 20px;
  height: 20px;
}
</style>