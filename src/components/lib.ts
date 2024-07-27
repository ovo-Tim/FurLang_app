import axios from "axios";
import { Command, ChildProcess } from '@tauri-apps/plugin-shell';

function setup_server(): [Command<string>, Promise<ChildProcess<string>>] {
    const sidecar_command = Command.sidecar("server/main");
    const exe = sidecar_command.execute();
    exe.then((s) => {
        console.log("stdout:", s.stdout);
        console.log("stderr:", s.stderr);
        console.log("code:", s.code);
        console.log("signal:", s.signal);
    })
    return [sidecar_command, exe];
}


// const axios_instance = axios.create({
//     baseURL: "http://" + server
// });

export interface info{
    learned: number,
    collocted: number
}
interface statistic{
    'learned': number,
    'familiar': number,
    'unfamiliar': number,
    'new': number
}

// async function FurPost(type: string, data: any){
//     try{
//         const msg = {type, data};
//         const res = (await axios_instance.post("", msg)).data;
//         return res;
//     }catch(e){
//         console.log("Post error: ", e)
//         return e;
//     }
// }

export default {
    // FurPost,
    setup_server,
}