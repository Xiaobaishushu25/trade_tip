import { createApp } from "vue";
import "./styles.css";
import App from "./App.vue";
import * as echarts from 'echarts/core';
import router from './router.ts'
import VueShortKey from 'vue3-shortkey';
import '@imengyu/vue3-context-menu/lib/vue3-context-menu.css'
import ContextMenu from '@imengyu/vue3-context-menu'
import {
    ToolboxComponent,
    TooltipComponent,
    GridComponent,
    VisualMapComponent,
    LegendComponent,
    BrushComponent,
    DataZoomComponent,
    MarkPointComponent
} from 'echarts/components';
import {
    CandlestickChart,
    LineChart,
    BarChart,
    CustomChart, ScatterChart
} from 'echarts/charts';
import { UniversalTransition } from 'echarts/features';
import { CanvasRenderer } from 'echarts/renderers';
import { GraphicComponent } from 'echarts/components';
echarts.use([
    ToolboxComponent,
    TooltipComponent,
    GridComponent,
    VisualMapComponent,
    LegendComponent,
    BrushComponent,
    DataZoomComponent,
    CandlestickChart,
    LineChart,
    BarChart,
    CanvasRenderer,
    UniversalTransition,
    CustomChart,
    GraphicComponent,
    MarkPointComponent,
    ScatterChart
]);

//
// import InlineSvgPlugin from 'vue-inline-svg';
import InlineSvg from 'vue-inline-svg';
let app = createApp(App);
app.component('inline-svg', InlineSvg);

app.use(ContextMenu).use(VueShortKey).use(router).mount('#app')
