<script setup lang="ts">
import { ref } from 'vue';
import { Command } from '@tauri-apps/plugin-shell';
let server_msg = ref("");
function disp(cmd: Command<string>){
    cmd.stdout.addListener("data", (data) => {
        server_msg.value = data;
    });
    cmd.stderr.addListener("data", (data) => {
        server_msg.value = data;
    });

}

defineExpose({
  disp
});
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
