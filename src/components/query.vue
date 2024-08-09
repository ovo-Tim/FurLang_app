<script setup lang="ts">
import { ref, watch } from 'vue';
import { FurPost } from './lib';
const query_word = ref('');

interface MwordInfo{
    'word': string, // lemmatized word
    'frequency': number,
    'familiarity': number,
    'last_used_date': string,
}

interface wordInfo{
    'word': string, // lemmatized word
    'frequency': number,
    'familiarity': number,
    'sentences': string[],
    'last_used_date': string,
    'notes': string[]
}

const infos = ref({} as MwordInfo);
const introductin_html = ref("");
const sentences = ref([] as string[]);
const notes = ref([] as string[]);

async function search(word: string){
    const info = (await FurPost("get_info", word)) as wordInfo;
    infos.value.familiarity = info.familiarity;
    infos.value.frequency = info.frequency;
    infos.value.last_used_date = info.last_used_date;
    infos.value.word = info.word;
    sentences.value = info.sentences;
    notes.value = info.notes;

    introductin_html.value = await FurPost("query_dicts", infos.value.word);
}

function update_notes(){
    console.log("Updating ", notes.value);

    FurPost("update_notes", [infos.value.word, notes.value]);
}
function update_sentences(){
    FurPost("update_sentences", [infos.value.word, sentences.value]);
}

watch(query_word, (word) => {
    infos.value = {} as MwordInfo;
    introductin_html.value = "";
    sentences.value = [];
    notes.value = [];
    search(word);
})

</script>
<template>
    <h3 style="font-family: 'Smiley Sans'; margin: 0.3em 1em;">Search</h3>
    <q-input v-model="query_word" debounce="500" filled placeholder="Search" hint="Search the word you learned." style="margin: 1.5em 0;">
        <template v-slot:append>
            <q-icon name="search" />
        </template>
    </q-input>
    <q-card class="qcard">
        <q-card-section>
            <h5>Introduction</h5>
            <div v-html="introductin_html" id="dict"></div>
        </q-card-section>
    </q-card>
    <q-card class="qcard">
        <q-card-section>
            <table>
                <tr>
                    <td>Word</td>
                    <td>{{ infos.word }}</td>
                </tr>
                <tr>
                    <td>Familiarity</td>
                    <td>{{ infos.familiarity }}</td>
                </tr>
                <tr>
                    <td>Frequency</td>
                    <td>{{ infos.frequency }}</td>
                </tr>
                <tr>
                    <td>Last used date</td>
                    <td>{{ infos.last_used_date }}</td>
                </tr>
            </table>
        </q-card-section>
    </q-card>
    <q-card class="qcard">
        <q-card-section>
            <q-list bordered padding style="margin-bottom: 0.5em;">
                <q-item-label header class="text-weight-bold bg-light">
                Your example sentence
                </q-item-label>

                <q-item v-for="sentence in sentences" :key="sentence" clickable v-ripple>
                    <q-item-section avatar>
                        <q-item-label>{{ sentence }}</q-item-label>
                    </q-item-section>

                    <q-item-section side top>
                        <q-btn @click="sentences.splice(sentences.indexOf(sentence), 1);update_sentences()" flat round dense icon="close" color="negative"/>
                    </q-item-section>
                </q-item>
            </q-list>

            <q-list bordered separator style="margin-top: 0.5em;">
                <q-item-label header class="text-weight-bold bg-light">
                Your notes
                </q-item-label>
                <q-item v-for="(_, index) in notes" :key="index" class="q-pa-md">
                <q-input v-model="notes[index]" dense outlined style="width: 100%;" />
                <q-btn @click="notes.splice(index, 1);update_notes()" color="negative" icon="close" flat round dense />
                </q-item>
                <q-item>
                <q-btn @click="notes.push('')" label="New note" color="primary" unelevated />
                </q-item>
            </q-list>
        </q-card-section>
    </q-card>
</template>
<style>
@font-face{
    font-family: "Smiley Sans";
    src: url('../assets/SmileySans-Oblique.ttf.woff2') format('woff2');
}
#dict > h1{
    height: 1.5em;
    font-size: 1.5em;
    margin-top: -1em;
    margin-bottom: 1.5em;
}
.qcard{
    margin: 1em auto;
}
h5{
    width: min-content;
    margin: 0 auto;
}
</style>