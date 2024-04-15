// history模式
import {
    createRouter,
    createWebHashHistory, RouteRecordRaw,
} from 'vue-router'

import Tool from './components/Tool.vue'
import CandleChart from './components/CandleChart.vue'

const routes: Array<RouteRecordRaw> = [
// 路由的默认路径
    {
        path:'/',
        redirect:"/CandleChart"
    },
    {
        path:'/candleChart',
        component: CandleChart,
    },
    {
        path:'/tool',
        component: Tool,
    }
]

// 创建路由对象
const router = createRouter({
    history: createWebHashHistory(),
    routes
})
export default router;