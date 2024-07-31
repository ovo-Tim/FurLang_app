<script setup lang="ts">
import mainPage from './components/mainPage.vue';
import splashscreen from './components/splashscreen.vue';
import { start_server, sharedVars } from './components/lib.ts';
import { onMounted, ref, watch } from 'vue';
import anime from 'animejs';

start_server();

onMounted(() => {
    const time = 2500;
    const loadingPage = document.getElementById('loadingPage');
    const mainPage = document.getElementById('mainPage');

    watch(() => sharedVars.server_inited, (_) => {
      anime({
        duration: time,
        direction: 'reverse',
        easings: 'easeInQuint',
        update: (anim) => {
          mainPage!.style.filter = `blur(${anim.progress * 16}px)`;
        }
      });

      anime({
        duration: time-300,
        easings: 'easeInQuint',
        update: (anim) => {
          console.log(anim.progress);
          loadingPage!.style.filter = `opacity(${anim.progress})`;
        },
        complete: () => {
          loadingPage!.remove();
        }
      })
    })
})

</script>

<template>
<splashscreen id="loadingPage"/>
<mainPage id="mainPage" />
</template>

<style>
#mainPage{
  filter: blur(16px);
}
#loadingPage{
  z-index: 1000;
  top: 0;
  left: 0;
  position: fixed;
  width: 100%;
  height: 100%;
}
</style>
