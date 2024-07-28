<script setup lang="ts">
import mainPage from './components/mainPage.vue';
import splashscreen from './components/splashscreen.vue';
import lib from './components/lib.ts';
import { onMounted, ref } from 'vue';
import { useQuasar } from 'quasar';
import { invoke } from '@tauri-apps/api/core';

const $q = useQuasar();
const splash = ref();
invoke('start_server').catch((err) => {
  $q.notify({
    type: 'negative',
    message: err
  })
});

</script>

<template>
<splashscreen id="loadingPage" ref="splash" />
<mainPage id="main" />
</template>

<style>
#loadingPage{
  z-index: 1000;
  top: 0;
  left: 0;
  position: fixed;
  width: 100%;
  height: 100%;
}
</style>
