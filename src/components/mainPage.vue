<script setup lang="ts">
import { ref } from 'vue';
import dashboard from './dashboard.vue';
const splitterModel = ref(20);
const tab = ref('overview');

</script>

<template>
<q-splitter v-model="splitterModel" id="mainPage">
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
                    <dashboard />
                </q-tab-panel>
            </q-tab-panels>
        </template>
</q-splitter>
</template>

<style lang="scss">

</style>
