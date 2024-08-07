<script setup>
import { FurPost, sharedVars } from './lib';

async function init(){
    const res = await FurPost("learned_count", {});
    sharedVars.learningInfo.learned = res[0];
    sharedVars.learningInfo.collocted = res[1];
    console.log("dashboard init");
}
defineExpose({
    init: init
});
</script>
<template>
<q-card>
    <q-card-section>
        <div id="dashboard-container">
            <q-card bordered class="dashboard-unit">
                <q-card-section>
                    <div class="db-text">{{ sharedVars.learningInfo.learned }}</div>
                    <p style="text-align: center; margin: 0;">Learned vocabulary</p>
                </q-card-section>
            </q-card>
            <q-card bordered class="my-card dashboard-unit">
                <q-card-section>
                    <div class="db-text">{{ sharedVars.learningInfo.collocted }}</div>
                    <p style="text-align: center; margin: 0;">Collocted vocabulary</p>
                </q-card-section>
            </q-card>

            <q-card style="flex-basis: 21em;" bordered class="my-card dashboard-unit">
                <q-card-section>

                </q-card-section>
            </q-card>
        </div>
    </q-card-section>
</q-card>
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
    font-size: 1.8rem;
}
</style>