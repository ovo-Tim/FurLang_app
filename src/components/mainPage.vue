<script setup lang="ts">
import { ref, watch } from 'vue';
import dashboard from './dashboard.vue';
import query from './query.vue';
import { sharedVars } from './lib.ts';
const splitterModel = ref(20);
const tab = ref('overview');

const _dashboard = ref();
function init() {
    console.log("mainPage init");
    _dashboard.value.init();
}
const text_conf = ref("");
watch(() => sharedVars.config, () => {
    text_conf.value = JSON.stringify(sharedVars.config, null, 2);
})

defineExpose({
    init: init
});

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
                    <dashboard ref="_dashboard" />
                </q-tab-panel>
                <q-tab-panel name="server">
                    <h3>Server log</h3>
                    <q-input standout readonly type="textarea" rounded style="width: 95%; margin: 0 auto;"
                        v-model="sharedVars.server_msg" autogrow />
                    <h3>Config</h3>
                    <q-input standout readonly type="textarea" rounded style="width: 95%; margin: 0 auto;"
                        v-model="text_conf" autogrow />
                </q-tab-panel>
                <q-tab-panel name="query">
                    <query />
                </q-tab-panel>
            </q-tab-panels>
        </template>
    </q-splitter>
</template>

<style lang="scss">
h3{
    margin-bottom: 0;
    margin-top: 0.5em;
}
</style>
