import axios, { Axios } from "axios";
import { reactive } from 'vue'
import { invoke } from '@tauri-apps/api/core';
import { useQuasar } from 'quasar';
const $q = useQuasar();

interface FurConf{
    db_path: string,
    excluded_words_path: string,
    statistic_path: string,
    port: number,
    dicts_path: string
}

export const sharedVars = reactive({
    server_msg: "",
    server_inited: false,
    config: {} as FurConf,
    conf_inited: false,
    base_url: "",
    learningInfo: {learned: 0, collocted: 0} as LearningInfo,
});

export interface LearningInfo{
    learned: number,
    collocted: number
}
interface statistic{
    'learned': number,
    'familiar': number,
    'unfamiliar': number,
    'new': number
}

interface CommandState {
    stdout: string,
    exit_code: number,
}

export function start_server(){
    invoke('start_server').catch((err) => {
        console.log(err);
    });

    get_state();
}

function get_state(){
    const interval = setInterval(()=>{
        invoke('get_state').then((_msg) => {
            const msg = _msg as CommandState;
            sharedVars.server_msg += msg.stdout;
            // console.log(sharedVars.server_msg);

            if (msg.stdout.includes('Start service')){
                sharedVars.server_inited = true;
            }else if (msg.exit_code != -1){
                sharedVars.server_inited = false;
                clearInterval(interval);
            }
        });
    }, 500);

}

export async function get_conf(){
    const msg = await invoke('get_config');
    sharedVars.config = msg as FurConf;
    sharedVars.conf_inited = true;
    sharedVars.base_url = `http://127.0.0.1:${sharedVars.config.port}`
}

export async function FurPost(type: string, data: any){
    try{
        const msg = {type, data};
        // const res = (await sharedVars.axios_instance!.post("", msg)).data;
        const req = await fetch(sharedVars.base_url, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json'
            },
            body: JSON.stringify(msg)
        });
        const res = await req.json();
        console.log("Post success:", res);

        return res;
    }catch(e){
        console.log(`Post {type: ${type}, data: ${data}} error: `, e);
        return e;
    }
}

export default {
    // FurPost,

}