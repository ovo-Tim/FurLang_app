<script setup lang="ts">
import { ref } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { useQuasar } from 'quasar';
const $q = useQuasar()

const server_msg = ref("");
interface CommandState {
    stdout: String,
    stderr: String,
    exit_code: number,
}
const getting_state = setInterval(()=>{
    invoke('get_state').then((_msg) => {
    const msg = _msg as CommandState;
    console.log(msg);
    server_msg.value += msg.stdout;
    server_msg.value += msg.stderr;
});
}, 500);
</script>

<template>
<div id="main">
    <h1 style="text-align: center;">Fur</h1>
    <q-input standout v-model="server_msg" readonly type="textarea" rounded style="width: 70%; margin: 0 auto;"/>
</div>
</template>

<style>
#main{
    /* backdrop-filter: blur(20px); */
}
</style>
