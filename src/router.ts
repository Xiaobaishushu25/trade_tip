// history模式
import {
    createRouter,
    createWebHashHistory, RouteRecordRaw,
} from 'vue-router'

import Tool from './components/Tool.vue'
import GroupTab  from "./components/GroupTab.vue";
import DataDetail from "./components/StockData/DataDetail.vue";
import MainLayout from "./components/MainLayout.vue";
import Setting from "./components/Setting.vue";
import TransactionRecord from "./components/TransactionRecord.vue";
import CanT from "./components/CanT.vue";
import Position from "./components/Position.vue";

const routes: Array<RouteRecordRaw> = [
    // 主界面的路由
    {
        path: '/',
        redirect: '/main/tab'
    },
    {
        path: '/main',
        component: MainLayout, // 主界面的布局组件
        children: [
            // 主界面下的子路由
            {
                path: 'tab', // 注意这里的path没有斜杠开头，表示它是/main的子路由
                component: GroupTab,
            },
            {
                path: 'detail',
                component: DataDetail,
            },
            // 其他主界面下的子路由...
        ]
    },
    // 工具界面的路由
    {
        path: '/tool',
        component: Tool, // 工具界面的布局组件
    },
    {
        path: '/setting',
        component: Setting, // 工具界面的布局组件
    },
    {
        path: '/record',
        component: TransactionRecord, // 工具界面的布局组件
    },
    {
        path: '/position',
        component: Position,
        //用路由传参简单的参数还好，但是复杂的可能会过长，不如用localStorage
        // path: '/cant',
        // component: CanT,
        // props: route => ({ combinedData: route.query.combinedData })
    },
    {
        path: '/cant',
        component: CanT,
        //用路由传参简单的参数还好，但是复杂的可能会过长，不如用localStorage
        // path: '/cant',
        // component: CanT,
        // props: route => ({ combinedData: route.query.combinedData })
    },
]

// 创建路由对象
const router = createRouter({
    history: createWebHashHistory(),
    routes
})
export default router;