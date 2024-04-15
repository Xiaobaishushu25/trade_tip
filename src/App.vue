<script setup lang="ts">
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
import {onMounted, ref} from "vue";
import axios from "axios";
import Header from "./components/Header.vue";
onMounted(()=>{
  window.addEventListener("contextmenu",  (e) => {e.preventDefault()},false)
})
const keyWord = ref("")
function querySearchAsync(key: string, cb: any){
  // console.log(key)
  // console.log(cb)
  // axios.get(`https://searchadapter.eastmoney.com/api/suggest/get?input=${key}&type=8&token=D43BF722C8E33BDC906FB84D85E326E8&markettype=&mktnum=&jys=&classify=&securitytype=&status=&count=4&_=1712919708063`).then((res)=>{
  axios.get(`https://search-codetable.eastmoney.com/codetable/search/web?clientVersion=lastest&keyword=${key}`).then((res)=>{
    console.log(res.data.result)
    cb(res.data.result)
  })
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
.tag{
  font-size: 11px;
  color: white;
  background-color: dodgerblue;
  height: 20px;
  width: 35px;
  text-align: center; /* 水平居中（如果需要）*/
  line-height: 20px;
}
.row{
  align-items: center; /* 垂直居中 */
  justify-content: center;
}
</style>
