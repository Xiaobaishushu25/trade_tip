import { createApp } from "vue";
import "./styles.css";
import App from "./App.vue";
import * as echarts from 'echarts/core';
import router from './router.ts'
import VueShortKey from 'vue3-shortkey';
import {
    ToolboxComponent,
    TooltipComponent,
    GridComponent,
    VisualMapComponent,
    LegendComponent,
    BrushComponent,
    DataZoomComponent,
} from 'echarts/components';
import {
    CandlestickChart,
    LineChart,
    BarChart,
} from 'echarts/charts';
import { UniversalTransition } from 'echarts/features';
import { CanvasRenderer } from 'echarts/renderers';
import Unicon from 'vue-unicons'
import { uniCopyAlt, uniCopyLandscape,uniWindowSection, uniWindowMaximize, uniCommentMonochrome,uniMultiplyMonochrome,uniMinusSquareFullMonochrome } from 'vue-unicons/dist/icons.js'
import {icons} from "./icons.js"
icons.push(

    uniCopyAlt,
    uniCopyLandscape,
    uniWindowSection,
    uniWindowMaximize,
    uniCommentMonochrome,
    uniMultiplyMonochrome,
    uniMinusSquareFullMonochrome
);
Unicon.add(icons)


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
    UniversalTransition
]);

//

let app = createApp(App);
app.use(Unicon).use(VueShortKey).use(router).mount('#app')
