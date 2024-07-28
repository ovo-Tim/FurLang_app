<script setup lang="ts">
import mainPage from './components/mainPage.vue';
import splashscreen from './components/splashscreen.vue';
import lib from './components/lib.ts';
import { onMounted, ref } from 'vue';
import { useQuasar } from 'quasar';
import { Command } from '@tauri-apps/plugin-shell';

const splash = ref();
const $q = useQuasar();
onMounted(()=>{
  const serverTask = Command.sidecar("server/main");
  splash.value.disp(serverTask);
  const taskExe = serverTask.execute()
  taskExe.then((res) => {
    console.log("Code: ", res.code);
    console.log("Stdout: ", res.stdout);
    console.log("Stderr: ", res.stderr);

    if (res.signal == 6) {
      $q.notify({
        type: 'negative',
        message: "Warning! The server has crashed, signal 6.(One possible solution is to move the server to another directory that don't have special character.)"
      });
    }
    $q.notify({
      type: 'negative',
      message: "Warning! The server has closed. Signal: " + res.signal
    });
  });
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
