<script setup>
import CtrlFormDone from './CtrlFormDone.vue'
import CtrlOthersList from './CtrlOthersList.vue'
import CtrlPwd from './CtrlPwd.vue'
import { ref } from 'vue'
import { useStore } from 'vuex'
import axios from 'axios'
import { notific_ctor_err_ctrl, notific_ctor_ok_ctrl, cmp_pwd_hash } from '@/js/elems/ctrl.js'

const store = useStore()

function newFormContent() {
    return {
        pwd: null,
    }
}
const form = ref(newFormContent())

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
        notific_ctor_ok_ctrl('вроде как успешно вырубились')
    }).catch(err => {
        var err = err.response.data
        notific_ctor_err_ctrl(`[${err.code}]: ${err.msg}`) 
    });
}

</script>

<template>
    <form class="ctrl-form" v-on:submit.prevent="onSubmit">
        <h4 class="ctrl-header">SHUTDOWN</h4>
        <CtrlPwd :form="form" :line="6" />
        <CtrlFormDone />
    </form>
    <br/>
    <CtrlOthersList />
</template>
