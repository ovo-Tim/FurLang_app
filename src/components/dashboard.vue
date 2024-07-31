<script setup>
import { sharedVars } from './lib';
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

            <q-card bordered class="my-card dashboard-unit">
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
    flex: 0 1 21em;
    margin: 0.5rem;
}
.db-text{
    color: $cyan-6;
    text-align: center;
    font-size: 1.8rem;
}
</style>