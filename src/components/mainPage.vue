<script setup lang="ts">
import { ref } from 'vue';
import { info } from './lib.ts';
const splitterModel = ref(20);
const tab = ref('');
const myInfo = ref({learned: 0, collocted: 0} as info);

</script>

<template>
<q-splitter v-model="splitterModel" id="main">
        <template v-slot:before>
            <q-tabs v-model="tab" vertical class="text-teal">
                <q-tab name="overview" icon="home" label="Overview" />
                <q-tab name="server" icon="dns" label="Server" />
                <q-tab name="query" icon="search" label="Query" />
            </q-tabs>
        </template>
        <template v-slot:after>
            <q-tab-panels v-model="tab" animated swipeable vertical transition-prev="jump-up" transition-next="jump-up">
                <q-tab-panel name="overview">
                    <q-card>
                        <q-card-section>
                            <div id="dashboard-container">
                                <q-card bordered class="dashboard-unit">
                                    <q-card-section>
                                        <div class="db-text">{{ myInfo.learned }}</div>
                                        <p style="text-align: center; margin: 0;">Learned vocabulary</p>
                                    </q-card-section>
                                </q-card>
                                <q-card bordered class="my-card dashboard-unit">
                                    <q-card-section>
                                        <div class="db-text">{{ myInfo.collocted }}</div>
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
                </q-tab-panel>
            </q-tab-panels>
        </template>
</q-splitter>
</template>

<style lang="scss">
#main{
    filter: blur(10px);
}
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
