<!--<script setup lang="ts">-->
<!--import {watch, ref, h} from "vue";-->
<!--import {invoke} from "@tauri-apps/api/core";-->
<!--import {NTag, SelectGroupOption, SelectOption} from "naive-ui";-->
<!--import InlineSvg from "vue-inline-svg";-->

<!--const keyWord = ref("ce")-->
<!--const nowStock = ref<{ code: string, name: string}>({code:"", name:""})-->
<!--const dialogFormVisible = ref(false)-->
<!--const showGroupMange = ref(false)-->
<!--// function querySearchAsync(key: string, cb: any){-->
<!--//   // console.log(key)-->
<!--//   // console.log(cb)-->
<!--//   if(key.length === 0){-->
<!--//     cb([])-->
<!--//     return-->
<!--//   }-->
<!--//   // axios.get(`https://searchadapter.eastmoney.com/api/suggest/get?input=${key}&type=8&token=D43BF722C8E33BDC906FB84D85E326E8&markettype=&mktnum=&jys=&classify=&securitytype=&status=&count=4&_=1712919708063`).then((res)=>{-->
<!--//   invoke("get_response",{url:`https://search-codetable.eastmoney.com/codetable/search/web?clientVersion=lastest&keyword=${key}`}).then((res)=>{-->
<!--//     // console.log(res)-->
<!--//     cb(JSON.parse(res).result)-->
<!--//   })-->
<!--//   // axios.get(`https://search-codetable.eastmoney.com/codetable/search/web?clientVersion=lastest&keyword=${key}`).then((res)=>{-->
<!--//   //   console.log(res.data.result)-->
<!--//   //   cb(res.data.result)-->
<!--//   // })-->
<!--// }-->
<!--const options = ref([])-->
<!--//下面这两个可以使用一个computed完成，但我还是分开写-->
<!--watch(keyWord,(newValue: string)=>{-->
<!--  querySearchAsync(newValue)-->
<!--})-->
<!--function querySearchAsync(key:string){-->
<!--  if(key.length === 0){-->
<!--    return-->
<!--  }-->
<!--  invoke<string>("get_response",{url:`https://search-codetable.eastmoney.com/codetable/search/web?clientVersion=lastest&keyword=${key}`}).then((res)=>{-->
<!--    options.value = JSON.parse(res).result.map((item: any)=>{-->
<!--      return {-->
<!--        label: item.shortName,-->
<!--        value: item.code,-->
<!--        market: item.market,-->
<!--        status: item.status,-->
<!--        securityTypeName: item.securityTypeName,-->
<!--      }-->
<!--    })-->
<!--  })-->
<!--}-->
<!--const handleSelect = (item: any) => {-->
<!--  console.log("点击了"+item)-->
<!--}-->

<!--function manageGroup(code: string, name: string){-->
<!--  showGroupMange.value = !showGroupMange.value-->
<!--  nowStock.value = {code:code,name:name}-->
<!--}-->

<!--const hideDialog = (ok: boolean) => {-->
<!--  dialogFormVisible.value = false-->
<!--  if (ok){-->
<!--    keyWord.value=""-->
<!--  }-->
<!--};-->
<!--// const  renderLabel= (info: { node: VNode, option: SelectOption | SelectGroupOption, selected: boolean }) => VNodeChild => [-->
<!--//   info.option.label as string,-->
<!--//   ' ',-->
<!--//   h(NTag, { size: 'small', type: 'info' }, { default: () => 'Email' })-->
<!--// ]-->
<!--const renderOption = (info: { node: VNode, option: SelectOption | SelectGroupOption, selected: boolean }) => {-->
<!--  const  option  = info.option;-->
<!--  console.log("选项是",option)-->
<!--  // 假设 option 是 SelectOption 类型，并且有 securityTypeName, shortName, 和 code 属性-->
<!--  // 如果 option 可能是 SelectGroupOption，你可能需要添加额外的逻辑来处理这种情况-->
<!--  return h('div', { class: 'row' }, [-->
<!--    h(NTag, { size: 'small', type: 'info' }, { default: () => `${option.securityTypeName}` }),-->
<!--    h('label', {}, [-->
<!--      `${option.label}(${option.value})`-->
<!--    ]),-->
<!--    h(InlineSvg, {-->
<!--      src: './src/assets/svg/add.svg',-->
<!--      class: 'min-icon add',-->
<!--      onClick: () => {-->
<!--        // 调用 manageGroup 方法，这里假设 manageGroup 是在父组件中定义的方法-->
<!--        // 你可能需要通过 info.node.context 或者其他方式访问 manageGroup 方法-->
<!--        // 这取决于你的组件结构和数据流-->
<!--        info.node.context.manageGroup(option.code, option.shortName);-->
<!--      }-->
<!--    })-->
<!--  ]);-->
<!--};-->
<!--</script>-->

<!--<template>-->
<!--&lt;!&ndash;  注意，他这里是v-model:value，使用v-model绑定不到&ndash;&gt;-->
<!--  <n-auto-complete-->
<!--      v-model:value="keyWord"-->
<!--      :clearable = "true"-->
<!--      :options="options"-->
<!--      placeholder="输入股票代码、名称、简拼或关键字"-->
<!--      @select="handleSelect"-->
<!--      :render-option="renderOption"-->
<!--      size="small"-->
<!--      style="width: 40%;margin-left: 50px"-->
<!--  >-->
<!--    <template #suffix>-->
<!--      <inline-svg src="./src/assets/svg/search.svg" class="min-icon"></inline-svg>-->
<!--    </template>-->
<!--  </n-auto-complete>-->
<!--</template>-->

<!--<style scoped>-->

<!--</style>-->