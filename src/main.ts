import { createApp } from "vue";
import "./styles.css";
import App from "./App.vue";
import * as echarts from 'echarts/core';
import {
    ToolboxComponent,
    ToolboxComponentOption,
    TooltipComponent,
    TooltipComponentOption,
    GridComponent,
    GridComponentOption,
    VisualMapComponent,
    VisualMapComponentOption,
    LegendComponent,
    LegendComponentOption,
    BrushComponent,
    BrushComponentOption,
    DataZoomComponent,
    DataZoomComponentOption
} from 'echarts/components';
import {
    CandlestickChart,
    CandlestickSeriesOption,
    LineChart,
    LineSeriesOption,
    BarChart,
    BarSeriesOption
} from 'echarts/charts';
import { UniversalTransition } from 'echarts/features';
import { CanvasRenderer } from 'echarts/renderers';

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

createApp(App).mount("#app");
