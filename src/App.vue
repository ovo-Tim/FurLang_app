<script setup lang="ts">
import mainPage from './components/mainPage.vue';
import splashscreen from './components/splashscreen.vue';
import { start_server, sharedVars } from './components/lib.ts';
import { onMounted, ref, watch } from 'vue';
import anime from 'animejs';

start_server();

onMounted(() => {
    const loadingPage = document.getElementById('loadingPage');
    const mainPage = document.getElementById('mainPage');
    let played = false;
    let beginAnimation = anime.timeline({
      easings: 'easeInQuint',
      duration: 2500,
      autoplay: false,
      direction: 'reverse',
    });

    beginAnimation.add({
        update: (anim) => {
          mainPage!.style.filter = `blur(${(anim.progress * 13)*0.01}px)`;
          if (played && !sharedVars.server_inited){  // If the server has stoped, reopen the splashscreen
            // beginAnimation.reverse();
            played = false;
          }
        }
      });

      beginAnimation.add({

        update: (anim) => {
          console.log(anim.progress);

          loadingPage!.style.filter = `opacity(${ anim.progress }%)`;
        },
        complete: () => {
          loadingPage!.remove();
        }
      });

    watch(() => sharedVars.server_inited, (_) => {
      if (sharedVars.server_inited){
        beginAnimation.play();
        played = true;
      }

    })
})

</script>

<template>
<splashscreen id="loadingPage"/>
<mainPage id="mainPage" />
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
