<script setup lang="ts">
import mainPage from './components/mainPage.vue';
import splashscreen from './components/splashscreen.vue';
import { start_server, sharedVars, get_conf } from './components/lib.ts';
import { onMounted, ref, watch } from 'vue';
import anime from 'animejs';

start_server();
const mainPageHander = ref();

onMounted(() => {
    const _loadingPage = document.getElementById('loadingPage');
    const _mainPage = document.getElementById('mainPage');
    let beginAnimation = anime.timeline({
      easings: 'easeInQuint',
      duration: 2500,
      autoplay: false,
      direction: 'reverse',
    });

    beginAnimation.add({
        update: (anim) => {
          _mainPage!.style.filter = `blur(${(anim.progress * 13)*0.01}px)`;
        }
      }, 0);

    beginAnimation.add({

      update: (anim) => {
        _loadingPage!.style.filter = `opacity(${ anim.progress }%)`;
      },
      complete: () => {
        _loadingPage!.remove();
      }
    }, 0);

    watch(() => sharedVars.server_inited,
      (_) => {
        if (sharedVars.server_inited){
          beginAnimation.play();
          get_conf().then(() => {
            mainPageHander.value.init();
          });

        }
    })
})
</script>

<template>
<splashscreen id="loadingPage"/>
<mainPage id="mainPage" ref="mainPageHander" />
</template>

<style>
#mainPage{
  filter: blur(13px);
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
