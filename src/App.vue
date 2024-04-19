<script setup lang="ts">
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
import {onMounted, ref} from "vue";
// import axios from "axios";
import Header from "./components/Header.vue";
import {StockInfo} from "./type.ts";
import {invoke} from "@tauri-apps/api/core";
import {store} from "./store.ts"
import {useRouter} from "vue-router";
// const stockInfo = ref<StockInfo[]>([]);
const router = useRouter()
onMounted(()=>{
  //https://router.vuejs.org/zh/guide/advanced/composition-api.html#%E5%9C%A8-setup-%E4%B8%AD%E8%AE%BF%E9%97%AE%E8%B7%AF%E7%94%B1%E5%92%8C%E5%BD%93%E5%89%8D%E8%B7%AF%E7%94%B1
  window.addEventListener("contextmenu",  (e) => {e.preventDefault()},false)
  router.push("/tab")
  invoke<StockInfo[]>("query_stock_info", {}).then(res => {
    console.log(res);
    store.stockInfo = res;
    store.stockInfo[0].price= 1.56
    console.log(store.stockInfo[0].price)
    // stockInfo.value = res;
  }).catch(err => {
    console.log(err);
  })
})
const keyWord = ref("")
function querySearchAsync(key: string, cb: any){
  // console.log(key)
  // console.log(cb)
  // axios.get(`https://searchadapter.eastmoney.com/api/suggest/get?input=${key}&type=8&token=D43BF722C8E33BDC906FB84D85E326E8&markettype=&mktnum=&jys=&classify=&securitytype=&status=&count=4&_=1712919708063`).then((res)=>{
  // axios.get(`https://search-codetable.eastmoney.com/codetable/search/web?clientVersion=lastest&keyword=${key}`).then((res)=>{
  //   console.log(res.data.result)
  //   cb(res.data.result)
  // })
}

const handleSelect = (item: any) => {
  console.log(item)
}
</script>

<template>
  <Header></Header>
  <div class="container">
<!--    <el-autocomplete-->
<!--        v-model="keyWord"-->
<!--        :clearable = "true"-->
<!--        :fetch-suggestions="querySearchAsync"-->
<!--        :trigger-on-focus="false"-->
<!--        placeholder="输入股票代码、名称、简拼或关键字"-->
<!--        @select="handleSelect"-->
<!--        value-key="shortName"-->
<!--    >-->
<!--    <template #default="{ item }">-->
<!--      <div class="row">-->
<!--        <div class="tag">{{ item.securityTypeName }}</div>-->
<!--        <label class="link">{{ `${item.shortName}(${item.code})` }}</label>-->
<!--      </div>-->
<!--    </template>-->
<!--    </el-autocomplete>-->
<!--    <keep-alive id="route" >-->
<!--      <router-view></router-view>-->
<!--    </keep-alive>-->
    <router-view v-slot="{ Component }">
      <keep-alive>
        <component :is="Component" />
      </keep-alive>
    </router-view>
<!--    <div class="column tip">-->
<!--      <label style="background-color: rgb(241, 241, 148)">2022-15-2/五</label>-->
<!--      <hr style="margin:0;"> &lt;!&ndash; 添加水平线并设置上下边距 &ndash;&gt;-->
<!--      <label>开盘：0.391</label>-->
<!--      <label>最高：0.391</label>-->
<!--      <label>最低：0.391</label>-->
<!--      <label>收盘：0.391</label>-->
<!--      <label>涨跌：0.391</label>-->
<!--      <label>涨幅：0.391</label>-->
<!--      <label>成交量：0.391</label>-->
<!--    </div>-->
<!--    <Tool></Tool>-->
  </div>
</template>

<style scoped>

</style>
