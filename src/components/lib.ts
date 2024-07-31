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
    axios_instance: undefined as Axios | undefined,
    learningInfo: {learned: 0, collocted: 0} as LearningInfo,
})

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
    stdout: String,
    exit_code: number,
}

export function start_server(){
    invoke('start_server').catch((err) => {
        $q.notify({
            type: 'negative',
            message: err
        })
    });
    setInterval(()=>{
        invoke('get_state').then((_msg) => {
            const msg = _msg as CommandState;
            console.log(msg);
            sharedVars.server_msg += msg.stdout;
            if (msg.stdout.includes('Start service')){
                sharedVars.server_inited = true;
                get_conf();
            }
        });
    }, 500);
}

function get_conf(){
    invoke('get_conf').then((_msg) => {
        sharedVars.config = _msg as FurConf;
        sharedVars.conf_inited = true;
        sharedVars.axios_instance = axios.create({
            baseURL: `http://127.0.0.1:${sharedVars.config.port}`
        })
    });
}

export async function FurPost(type: string, data: any){
    try{
        const msg = {type, data};
        const res = (await sharedVars.axios_instance!.post("", msg)).data;
        return res;
    }catch(e){
        console.log("Post error: ", e);
        $q.notify({
            type: 'negative',
            message: "Request failed:" + e
        })
        return e;
    }
}

export default {
    // FurPost,

}