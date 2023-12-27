<script setup>
import CtrlFormText from './CtrlFormText.vue'
import CtrlFormDone from './CtrlFormDone.vue'
import CtrlOthersList from './CtrlOthersList.vue'
import LineN from './CtrlFormLine.vue'
import { ref } from 'vue'
import { useStore } from 'vuex'
import axios from 'axios'
import { notific_ctor_err_ctrl, cmp_pwd_hash } from '@/js/elems/ctrl.js'
import { notific_ctor, NOTIFIC_TY_INFO } from "@/js/elems/notific";

const store = useStore()

function newFormContent() {
    return {
        pwd: null,
    }
}
const form = ref(newFormContent())

function notific_ctor_ok_local() {
    let msg = 'вроде как успешно вырубились'
    return notific_ctor(NOTIFIC_TY_INFO, msg, 2_000, true, true)
}
function onSubmit() {
    let pwd_hash = cmp_pwd_hash(form.value.pwd, '#')
    if (!pwd_hash) { return }

    let data = { pwd_hash, }
    
    form.value = newFormContent();

    let url = '/~~ctrl~~/shutdown'
    axios({
        url: store.getters.getPort + url,
        method: 'post',
        data,
    }).then(_ => {
        notific_ctor_ok_local()
    }).catch(err => {
        var err = err.response.data
        notific_ctor_err_ctrl(`[${err.code}]: ${err.msg}`) 
    });
}

</script>

<template>
    <form class="ctrl-form" v-on:submit.prevent="onSubmit">
        <h4 class="ctrl-header">SHUTDOWN</h4>
        <CtrlFormText v-model="form.pwd" :tab="1" :maxLen="8 + 1 + 12" placeholder="<nonce><space><pwd>">
            nonce&amp;pwd<LineN :n="6" />:
        </CtrlFormText>

        <CtrlFormDone />
    </form>
    <br/>
    <CtrlOthersList />
</template>
