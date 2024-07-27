<script setup lang="ts">
import mainPage from './components/mainPage.vue';
import splashscreen from './components/splashscreen.vue';
import lib from './components/lib.ts';
import { onMounted, ref } from 'vue';
import { useQuasar } from 'quasar'

const qua = useQuasar();

let [serverTask, taskExe] = lib.setup_server();
const splash = ref();
onMounted(()=>{
  splash.value.disp(serverTask);
});
taskExe.then((res) => {
  if (res.signal == 6) {
    qua.notify({
      type: 'negative',
      message: "Warning! The server has crashed, signal 6.(One possible solution is to move the server to another directory that don't have special character.)"
    })
  }
})
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
