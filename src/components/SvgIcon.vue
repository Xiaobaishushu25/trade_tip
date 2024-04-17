<script setup lang="ts">
import { computed, defineProps } from 'vue'

// 定义组件属性
const props = defineProps({
  prefix: {
    type: String,
    default: 'svg',
  },
  name: {
    type: String,
    required: true,
  },
  color: {
    type: String,
    default: '#333',
  },
})
// 计算属性
const symbolId = computed(() => `#${props.prefix}-${props.name}`)
</script>

<template>
  <svg aria-hidden="true" class="icon">
    <use :xlink:href="symbolId" :fill="color" />
  </svg>
</template>

<style scoped>
.icon{
  height: 32px;
  width: 32px;
}
</style>