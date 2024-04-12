<script setup lang="ts">

</script>


<template>
<div class="main" id="draggable-main">
  <div class="two-column">
    <img src="../assets/hline.svg" class="icon" alt="水平直线" />
    <img src="../assets/line.svg" class="icon" alt="水平直线" />
    <img src="../assets/hpartline.svg" class="icon" alt="水平直线" />
    <img src="../assets/A.svg" class="icon" alt="水平直线" />
    <img src="../assets/up.svg" class="icon" alt="水平直线" />
    <img src="../assets/down.svg" class="icon" alt="水平直线" />
  </div>
</div>
</template>

<style scoped>
.main{
  width: 80px;
  background-color: #ecece4;
}
.two-column{
  display: grid;
  grid-template-columns: repeat(2, 1fr); /* 创建两列 */
  grid-gap: 2px; /* 列与列之间的间距 */
}
.icon{
  width: 16px;
  height: 16px;
}
</style>