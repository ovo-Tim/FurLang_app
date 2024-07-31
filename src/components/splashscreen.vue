<script setup lang="ts">
import { ref } from 'vue';
import { sharedVars } from './lib.ts';

</script>

<template>
<div id="slashscreen">
    <h1 style="text-align: center;">Fur</h1>
    <q-input standout v-model="sharedVars.server_msg" readonly type="textarea" rounded style="width: 70%; margin: 0 auto;"/>
</div>
</template>

<style>
</style>
