<script setup>
import CtrlFormText from './CtrlFormText.vue'
import CtrlFormCheckbox from './CtrlFormCheckbox.vue'
import CtrlFormDone from './CtrlFormDone.vue'
import CtrlOthersList from './CtrlOthersList.vue'
import LineN from './CtrlFormLine.vue'
import { ref, computed, defineProps } from 'vue'
import { useStore } from 'vuex'
import axios from 'axios'
import { sha3_256 } from 'js-sha3';
import { notific_ctor, NOTIFIC_TY_ERR, NOTIFIC_TY_INFO } from "@/js/elems/notific";

const store = useStore()

const props = defineProps({
    isSave: {type: Boolean, required: true},
})

const header = computed(() => {
    return (props.isSave) ? "FULL SAVE" : "FULL LOAD"
})
const lineN = computed(() => {
    return (props.isSave) ? 1 : 2
})


function newFormContent(single_file = true) {
    return {
        pwd: null,
        save_name: null,
        single_file,
    }
}
const form = ref(newFormContent())


function notific_ctor_err_local(msg) {
    // return notific_ctor(NOTIFIC_TY_ERR, msg, top = true, left = true)
    return notific_ctor(NOTIFIC_TY_ERR, msg, 2_000, true, true)
}
function notific_ctor_ok_local() {
    let msg = (props.isSave ? 'сохранено' : 'загружено')
    msg = `успешно ${msg}!`
    return notific_ctor(NOTIFIC_TY_INFO, msg, 2_000, true, true)
}

function onSubmit() {
    let save_name = form.value.save_name
    if (!save_name || save_name.length == 0) {
        notific_ctor_err_local("empty save name`")
        return
    }

    let pwd_hash = form.value.pwd
    if (pwd_hash) {
        pwd_hash = pwd_hash.split(' ')
        if (pwd_hash.length != 2) {
            notific_ctor_err_local("`nonce&pwd` must contain only `nonce` and `pwd` and be splited with space")
            return
        }
        pwd_hash = pwd_hash[0] + save_name + pwd_hash[1]
        pwd_hash = sha3_256(pwd_hash)
    } else {
        notific_ctor_err_local("empty `nonce&pwd`")
        return
    }

    let single_file = form.value.single_file;

    let data = {
        pwd_hash,
        save_name,
        single_file,
    }
    
    form.value = newFormContent(single_file);

    let url = '/~~ctrl~~/' + (props.isSave ? 'full_save' : 'full_load')
    axios({
        url: store.getters.getPort + url,
        method: 'post',
        data,
    }).then(_ => {
        notific_ctor_ok_local()
    }).catch(err => {
        var err = err.response.data
        notific_ctor_err_local(`[${err.code}]: ${err.msg}`) 
    });
}

</script>

<template>
    <form class="ctrl-form" v-on:submit.prevent="onSubmit">
        <h4 class="ctrl-header" v-html="header" />
        <CtrlFormText v-model="form.pwd" :tab="1" :maxLen="8 + 1 + 12" placeholder="<nonce><space><pwd>">
            nonce&amp;pwd<LineN :n="lineN" />:
        </CtrlFormText>

        <CtrlFormText v-model="form.save_name" :tab="2">save name:</CtrlFormText>
        <CtrlFormCheckbox v-model="form.single_file" :tab="3">single_file:</CtrlFormCheckbox>

        <CtrlFormDone />
    </form>

    <br/>
    <CtrlOthersList />
</template>

<style>
.ctrl-form {
    width: fit-content;
    padding: 3px 1.5ch;
    margin-left: 0.6em;
    margin-top: calc(2ch + 4em);
    background-color: var(--r-col-bg-dark);
}
.ctrl-header {
    text-decoration: underline;
    color: var(--r-col-blue);
    /* border-bottom: 1px solid var(--r-col-blue); */
}
.ctrl-label {
    display: inline-block;
    min-width: 13ch;
    color: var(--r-col-blue);
}
</style>