<script setup>
import { provide, ref, onMounted } from 'vue';
import { FurPost, sharedVars } from './lib';
import { use } from "echarts/core";
import {
  TitleComponent,
  ToolboxComponent,
  TooltipComponent,
  GridComponent,
  LegendComponent
} from 'echarts/components';
import { LineChart } from 'echarts/charts';
import { UniversalTransition } from 'echarts/features';
import { CanvasRenderer } from 'echarts/renderers';
import { generateGradientStackedAreaChartOptionFromStatistic } from './charts';
import * as echarts from 'echarts';

async function init(){
    const res = await FurPost("learned_count", {});
    sharedVars.learningInfo.learned = res[0];
    sharedVars.learningInfo.collocted = res[1];
    InitChart();
    console.log("dashboard init");
}
defineExpose({
    init: init
});

async function InitChart(){
    const data = await FurPost("get_statistic", {});
    const chartOption = generateGradientStackedAreaChartOptionFromStatistic(data);
    const chartdom = document.getElementById('chart');
    const chart = echarts.init(chartdom);
    chart.setOption(chartOption);

    const resizeObserver = new ResizeObserver(entries => {
        console.log("resize");

        chart.resize();
    });
    resizeObserver.observe(chartdom);
}

use([
  TitleComponent,
  ToolboxComponent,
  TooltipComponent,
  GridComponent,
  LegendComponent,
  LineChart,
  CanvasRenderer,
  UniversalTransition
]);


</script>
<template>
<q-card>
    <q-card-section>
        <div id="dashboard-container">
            <q-card bordered class="dashboard-unit">
                <q-card-section class="text-parent">
                    <div>
                        <div class="db-text">{{ sharedVars.learningInfo.learned }}</div>
                        <p style="text-align: center; margin: 0;">Learned vocabulary</p>
                    </div>

                </q-card-section>
            </q-card>
            <q-card bordered class="my-card dashboard-unit">
                <q-card-section class="text-parent">
                    <div>
                        <div class="db-text">{{ sharedVars.learningInfo.collocted }}</div>
                        <p style="text-align: center; margin: 0;">Collocted vocabulary</p>
                    </div>
                </q-card-section>
            </q-card>
        </div>
        <div id="chart" class="my-card" style="height: 35em;"></div>
    </q-card-section>
</q-card>
<q-btn push color="grey" text-color="white" icon="refresh" style="float: right;" @click="init()"/>
</template>
<style lang="scss">
#dashboard-container{
    display: flex;
    flex-wrap: wrap;
    justify-content: center;
}
.dashboard-unit{
    flex: 1 1 18em;
    margin: 0.5rem;
}
.db-text{
    color: $cyan-6;
    text-align: center;
    font-size: 2.5rem;
}
.text-parent{
    display: flex;
    align-items: center;
    height: 100%;
    justify-content: center;
}
</style>