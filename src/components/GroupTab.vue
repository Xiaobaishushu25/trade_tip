<script setup lang="ts">
import {onMounted,ref,watch} from "vue";
import {invoke} from "@tauri-apps/api/core";
import {store} from "../store.ts";
import {StockGroup, StockInfoG} from "../type.ts";
import {VueDraggable} from "vue-draggable-plus";
import InlineSvg from "vue-inline-svg";
import {listen} from "@tauri-apps/api/event";
import {successNotification} from "../utils.ts";

const activeName = ref('')
const dialogVisible = ref(false)
const buttonVisibilities = ref<boolean[]>()

onMounted(async () => {
  query_all_groups();
  query_box();
  await listen("graphic_change", () => {
    query_box();
  })
})
// computed(() => {
//   if (store.stockGroups.length > 0) {
//     return store.stockGroups.map(group => group.name);
//   } else {
//     return [];
//   }
// })
// const stockGroupNames = computed(() => {
//   // return store.stockGroups.map(group => ({name:group.name,index:group.index}))
//   return store.stockGroups.map(group => ({...group}))
//   // ... 类似的逻辑，但是你可能需要直接从 store 中解构出 stockGroups
// })
const stockGroupNames = ref<StockGroup[]>()
// watch(()=>store.stockGroups, () => {
watch(()=>store.stockGroups.length, () => { //监听长度变化比深度监听性能更好吧？
  stockGroupNames.value = store.stockGroups.map(group => ({...group}));
// },{immediate:true,deep:true})
},{immediate:true})
function query_all_groups(oldActiveName:string|null=null){
  invoke<StockGroup[]>("query_all_groups",{}).then(res => {
    store.stockGroups = res;
    if (oldActiveName !== null){
      activeName.value = oldActiveName;
    }else {
      activeName.value = store.stockGroups[0].name;
    }
    buttonVisibilities.value = store.stockGroups.map(() => false) // 初始化按钮可见性数组
    console.log("给全局状态赋值",store.stockinfoGs)
  }).catch(err => {
    console.log(err)
  });
}
function query_box(){
  invoke<Record<string, number[]>>("query_box",{}).then(res => {
    store.boxData = res
  }).catch(err => {
    console.log(err)
  })
}
watch(activeName, () => {
  let groupName = activeName.value;
  store.activeGroup = groupName;
  invoke("query_live_stocks_data_by_group_name",{groupName: groupName}).catch(err => {
    console.log(err);
  });
  invoke<StockInfoG[]>("query_stocks_by_group_name", {name: groupName}).then(res => {
    store.stockinfoGs = res;
    // console.log("给全局状态赋值",store.stockinfoGs)
  }).catch(err => {
    console.log(err);
  })
},{immediate:true})

function updateGroup(){
  dialogVisible.value = false;
  let changeGroups = []
  for (let i = 0; i < stockGroupNames.value!.length; i++) {
    if (stockGroupNames.value![i].index!=store.stockGroups[i].index){
      stockGroupNames.value![i].index = store.stockGroups[i].index;
      changeGroups.push(i);
    }
    // store.stockGroups[i].index = i;
  }
  let oldActiveName = activeName.value
  invoke("update_groups",{groups: stockGroupNames.value}).then(()=>{
  // invoke("update_groups",{groups:stockGroupNames.value}).then(()=>{
    console.log(`更新分组成功`);
    query_all_groups(oldActiveName);
    store.stockGroups.forEach((item) => {
      if (changeGroups.includes(item.index)){
        item.stocks_change = !item.stocks_change
      }
    })
    successNotification("更新分组排序成功");
  }).catch(err => {
    console.log(err);
  })
}
function remove(name:string){
  if (name=="全部"||name=="持有"){
    return;
  }
  invoke("delete_group",{name: name}).then(()=>{
    const index = store.stockGroups.findIndex(group => group.name === name); // 假设每个组都有一个name属性
    if (index !== -1) {
      store.stockGroups.splice(index, 1); // 移除找到的元素
    }
    console.log(`删除${name}成功`)
  }).catch(err => {
    console.log(err);
  })
}
function judgeTab(activeName:string){
  return activeName != '设置';
}
</script>

<template>
  <div class="tab-container">
    <el-tabs v-model="activeName"  tab-position="bottom" :before-leave="judgeTab"  >
      <el-tab-pane
          v-for="(group, index) in store.stockGroups"
          :key="index"
          :label="`${group.name}`"
          :name="`${group.name}`"
          class="tab-pane"
      >
<!--        <StockTable :stocks_change="group.stocks_change" :group-name="group.name" :active-name="activeName"></StockTable>-->
        <StockTable2 :stocks_change="group.stocks_change" :group-name="group.name" :active-name="activeName"></StockTable2>
      </el-tab-pane>
      <el-tab-pane name="设置">
        <template #label>
          <el-tooltip content="分组管理" placement="bottom" effect="light" :show-arrow="false">
            <inline-svg src="../assets/svg/menu.svg" class="min-icon" @click="dialogVisible = true"></inline-svg>
          </el-tooltip>
        </template>
<!--        <el-tooltip content="分组管理" placement="bottom" effect="light" :show-arrow="false">-->
<!--          <template #label>-->
<!--            &lt;!&ndash;          <inline-svg src="./src/assets/svg/menu.svg" class="min-icon" @click="dialogVisible = true"></inline-svg>&ndash;&gt;-->
<!--            <inline-svg src="../assets/svg/menu.svg" class="min-icon" @click="dialogVisible = true"></inline-svg>-->
<!--          </template>-->
<!--        </el-tooltip>-->
      </el-tab-pane>
    </el-tabs>
  </div>
  <el-dialog v-model="dialogVisible" :show-close="false" :draggable="true" width="250" align-center style="padding: 0">
    <template #header="{ }">
      <div class="my-header">
        <label style="font-size: 14px;margin-left: 15px;font-family:sans-serif">分组管理(可拖拽排序)</label>
<!--        <inline-svg src="./src/assets/svg/close.svg" class="small-close"  @click.left="dialogVisible=false"></inline-svg>-->
        <inline-svg src="../assets/svg/close.svg" class="small-close"  @click.left="dialogVisible=false"></inline-svg>
      </div>
    </template>
    <el-scrollbar ref="scrollbarRef" max-height="200px" style="display: flex;align-items: center; justify-content: center;" >
      <VueDraggable ref="el" v-model="stockGroupNames" class="group-column">
        <div v-for="item in stockGroupNames" :key="item.index" style="cursor: move">
          <div class="container" @click="buttonVisibilities[item.index] = false" v-bind:class="{ animate: buttonVisibilities[item.index] }">
            {{ item.name }}
            <svg t="1707745507392" class="icon spot"  @click.stop="buttonVisibilities[item.index] = !buttonVisibilities[item.index]"     viewBox="0 0 1024 1024" version="1.1" xmlns="http://www.w3.org/2000/svg" p-id="12062" width="24" height="24"><path d="M510.653 931.287z m0 92.713a92.708 92.708 0 1 0 0-185.426 92.713 92.713 0 0 0 0 185.426M512 512z m0 92.713a92.718 92.718 0 1 0 0-185.431 92.718 92.718 0 0 0 0 185.431m1.347-512z m0 92.713c51.205 0 92.713-41.508 92.713-92.713C606.06 41.508 564.552 0 513.347 0s-92.719 41.508-92.719 92.713c0 51.205 41.513 92.713 92.719 92.713" p-id="12063"></path></svg>
            <button v-show="buttonVisibilities[item.index]" class="red-button" @click.stop="remove(item.name)">删除</button>
          </div>
        </div>
      </VueDraggable>
    </el-scrollbar>

    <div class="dialog-footer right" style="margin-top: 20px">
      <el-button class="dialog-button" @click="dialogVisible=false">取消</el-button>
      <el-button class="dialog-button dialog-confirm" type="primary" @click="updateGroup">
        确定
      </el-button>
    </div>
  </el-dialog>
</template>

<style>
/*.el-dialog__header{
  margin-top: 0;
  margin-left: 0;
  margin-right: 0;
  width: 100%;
}*/
.dialog-button{
  height: 25px;
  width: 35px;
  font-size: 13px;
  background: rgba(229, 219, 219, 0.85);
  color:black;
  font-weight: bold;
  border-color: #9170b000;
}
.dialog-confirm{
  color: #9d6a09;
}
.min-icon{
  outline: none!important;
}
.right{
  display: flex;
  justify-content: right;
  margin-right: 10px;
  margin-bottom: 10px;
}

.spot{
  margin-left: 30px;
  height: 20px;
  fill: gray;
}
.spot:hover{
  cursor: pointer;
}
.tab-container{
  height: 100%;
}
.el-tabs__nav-scroll{
  padding-left: 20px;
}
.tab-pane{
  min-height: 200px;
}
.min-icon:hover path{
  color: green;
  fill: green;
  stroke: green;
}
.group-column{
  display: flex;
  flex-direction: column;
  gap: 10px;
}

.container {
  display: grid;
  grid-template-columns: repeat(2, 1fr); /* 创建两列 */
  grid-gap: 2px; /* 列与列之间的间距 */
  /* 初始状态，没有平移 */
  transform: translateX(0);
  margin-left: 40px;
  /* 定义过渡效果 */
  transition: transform 0.3s ease-in-out;
}

.container > * {
  margin-right: 10px; /* 为每个组件之间添加一些间距 */
}
.container.animate {
  transform: translateX(-30px);
}
.red-button {
  position: absolute;
  right: 0; /* 初始时放在视线外 */
  top: 0;
  background-color: #ee2727;
  color: white;
  border: none;
  padding: 5px;
  cursor: pointer;
  transition: left 0.5s ease; /* 添加过渡效果，持续时间为0.5秒 */
}
</style>