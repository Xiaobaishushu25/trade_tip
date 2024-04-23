// history模式
import {
    createRouter,
    createWebHashHistory, RouteRecordRaw,
} from 'vue-router'

import Tool from './components/Tool.vue'
import CandleChart from './components/CandleChart.vue'
import NewCandleChart from './components/NewCandleChart.vue'
import CandleChartNewNew from './components/CandleChartNewNew.vue'
import GroupTab  from "./components/GroupTab.vue";

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
        path:'/newCandleChart',
        component: NewCandleChart,
    },
    {
        path:'/newnewCandleChart/:code',
        name:'CandleChartNewNew',
        component: CandleChartNewNew,
    },
    {
        path:'/tool',
        component: Tool,
    },
    {
        path:'/tab',
        component: GroupTab,
    }
]

// 创建路由对象
const router = createRouter({
    history: createWebHashHistory(),
    routes
})
export default router;