import axios from "axios";

const axios_instance = axios.create({
    baseURL: "http://" + server
});

interface info{
    learned: number,
    collocted: number
}
interface statistic{
    'learned': number,
    'familiar': number,
    'unfamiliar': number,
    'new': number
}

async function FurPost(type: string, data: any){
    try{
        const msg = {type, data};
        const res = (await axios_instance.post("", msg)).data;
        return res;
    }catch(e){
        console.log("Post error: ", e)
        return e;
    }
}