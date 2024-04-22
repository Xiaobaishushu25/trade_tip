<script setup lang="ts">
import {onMounted, ref, nextTick, watch} from "vue";
import {invoke} from "@tauri-apps/api/core";
import {store} from "../../store.ts";
import {StockGroup} from "../../type.ts";

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
let isNew = false
const groupList = ref<string []>([])
const initSelectGroups= ref<string []>([])
const selectGroups = ref<string []>([])
const newName = ref("")
const isEditing = ref(false)
const inputRef = ref()
const scrollbarRef = ref()
const emit = defineEmits([ "hideDialog" ]);
const hideDialog = (ok:boolean) => {
  emit('hideDialog',ok);
  // emit('hideDialog');
  if (ok) {
    //update_stock_groups函数要求is_new==true和group_names长度为0不同时出现
    if (!(isNew&&selectGroups.value.length==0)){
      invoke("update_stock_groups", {isNew:isNew,code: props.code,name: props.name, groupNames: selectGroups.value}).then((res) => {
        const changes = findChangedStrings(initSelectGroups.value,selectGroups.value)
        changes.forEach((change) => {
          store.stockGroups.forEach((item) => {
            if (item.name==change){
              item.stocks_change = !item.stocks_change
            }
          })
        })
        console.log(res)
        initSelectGroups.value = selectGroups.value
      }).catch((err) => {
        console.log(err)
      })
    }
  }
}
// 监听code变化,因为是props，所以需要用这种方式
watch(() => props.code, (_) => {
  updateSelectGroup()
});
//选择分组变化时检查是否选择了全部，如果没有，则清空（全部是必选）
watch(selectGroups,(newValue,_)=>{
  if (!newValue.includes("全部"))
    selectGroups.value.length=0
})
//初始分组变化时检查长度是否为0，如果是，则isNew为true
watch(initSelectGroups,(newValue,_)=>{
  isNew = newValue.length == 0;
})
onMounted(() => {
  groupList.value = store.stockGroups.map((item) => {
    if (item.name!="持有")
      return item.name
    return ""
  }).filter((item) => {
    return item!=""
  })
  updateSelectGroup()
  // console.log(`allgroupList是`+store.stockGroups)
  // console.log(`allgroupList是`+store.stockGroups.length)
  // console.log(`groupList是`+groupList.value)
})
function updateSelectGroup() {
  invoke<string[]>("query_groups_by_code", {code: props.code}).then((res) => {
    selectGroups.value = res
    initSelectGroups.value = res
    // selectGroups.value?.push("全部")
    // console.log("selectGroups是"+selectGroups.value)
    // console.log("selectGroups是"+selectGroups.value.length)
  }).catch((err) => {
    console.log(err)
  })
}
async function showInput(){
  isEditing.value = true
  await nextTick()
  inputRef.value.focus()
  // nextTick(() => {
  //   inputRef.value.focus()
  // })
}
async function newGroup() {
  let value = newName.value;
  if (value!="") {
    invoke("create_group",{name:value}).then(async (number) => {
      groupList.value.push(value)
      store.stockGroups.push(<StockGroup>{index: number, name: value, stocks_change: false})
      await nextTick()
      scrollbarRef.value.scrollTo({position:'bottom', behavior: 'smooth'}) // 滚动到底部
      // store.stockGroups.push()
    }).catch((e)=>{
      console.log(e)
    })
  }
  isEditing.value = false
  newName.value = ""
  console.log(newName.value)
}
function disableSelect(name:string){
  if (name=="全部"){
    return false
  }else{
    return !selectGroups.value.includes("全部");
  }
}
function findChangedStrings(a:string[], b:string[]) {
  // const changes = {
  //   added: [],    // 存放增加的字符串
  //   removed: []   // 存放删除的字符串
  // };
  const changes:string[] = [];
  // 找出数组a中存在但b中不存在的字符串
  a.forEach(str => {
    if (!b.includes(str)) {
      changes.push(str);
    }
  });
  // 找出数组b中存在但a中不存在的字符串
  b.forEach(str => {
    if (!a.includes(str)) {
      changes.push(str);
    }
  });
  return changes;
}
</script>

<template>
  <div class="container">
    <n-space justify="start">
      <label  style="font-weight: bold;font-size: 15px">选择自选分组</label>
    </n-space>
    <n-flex vertical justify="center" align="center">
      <n-scrollbar ref="scrollbarRef" style="max-height: 230px">
        <n-flex vertical justify="center" align="center">
<!--          <el-checkbox-group v-model="selectGroups" class="column"  >-->
<!--            <el-checkbox-->
<!--                v-for="name in groupList"-->
<!--                :key="name"-->
<!--                :label="name"-->
<!--                :value="name"-->
<!--                :disabled="disableSelect(name)"-->
<!--                size="large"-->
<!--                style="transform: scale(1.15);"-->
<!--            ></el-checkbox>-->
<!--          </el-checkbox-group>-->
          <n-checkbox-group v-model:value="selectGroups">
            <n-flex vertical justify="center">
              <n-checkbox
                  v-for="name in groupList"
                  :label="name"
                  :value="name"
                  :disabled="disableSelect(name)"
                  size="large"
              />
            </n-flex>
          </n-checkbox-group>
        </n-flex>
      </n-scrollbar>
      <n-flex @click.left="showInput" style="height:23px;" align="center">
        <inline-svg src="./src/assets/svg/add.svg" class="min-icon"></inline-svg>
        <n-input
            v-model="newName"
            v-if="isEditing"
            @blur="isEditing = false"
            ref="inputRef"
            class="input-field"
            placeholder="输入新分组名称"
            @keydown.enter = "newGroup()"
            type="text"
            size="small"
            style="width: 130px"
        />
        <label  v-if="!isEditing" style="color: orange;cursor: pointer">新建分组</label>
      </n-flex>
    </n-flex>
    <n-flex justify="end" style="margin-top: 20px">
      <n-button size="tiny" strong secondary type="tertiary" @click="hideDialog(false)">取消</n-button>
      <n-button size="tiny" strong secondary type="success" @click="hideDialog(true)">确定</n-button>
    </n-flex>
  </div>
</template>
<style>
</style>

<style scoped>
.container{
  width: 280px;
  height: 320px;
}
</style>