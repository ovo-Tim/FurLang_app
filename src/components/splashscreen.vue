<script setup lang="ts">
import anime from 'animejs';
import { sharedVars } from './lib.ts';
import { onMounted } from 'vue';

onMounted(() => {
    anime({
        rotate: 360,
        duration: 1750,
        loop: true,
        easing: 'easeInOutBounce',
        targets: '#server-icon',
    })
});
</script>

<template>
<div id="slashscreen">
    <h1 id="title" style="text-align: center;">Furry Language</h1>
    <div style="width: fit-content; margin: 2em auto;display: flex;justify-content: space-between;align-items: center; ">
        <img id="server-icon" style="width: 5em;" src="../assets/server.svg"/>
        <p style="font-size: 1.8em;margin: 0 0.8em;">Server loading...</p>
    </div>
    <q-input standout readonly type="textarea" rounded style="width: 70%; margin: 0 auto;" v-model="sharedVars.server_msg"/>
</div>
</template>

<style>
@font-face{
    font-family: "Smiley Sans";
    src: url('../assets/SmileySans-Oblique.ttf.woff2') format('woff2');
}
#title{
    font-family: "Smiley Sans";
}
</style>
