<script setup>
import CtrlLogo from './CtrlLogo.vue'
import CtrlFormText from './CtrlFormText.vue'
import CtrlFormCheckbox from './CtrlFormCheckbox.vue'
import CtrlFormDone from './CtrlFormDone.vue'
import CtrlOthersList from './CtrlOthersList.vue'
import CtrlPwd from './CtrlPwd.vue'
import { ref, computed, defineProps } from 'vue'
import { useStore } from 'vuex'
import axios from 'axios'
import { notific_ctor_err_ctrl, notific_ctor_ok_ctrl, cmp_pwd_hash } from '@/js/elems/ctrl.js'

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

function onSubmit() {
    let save_name = form.value.save_name
    if (!save_name || save_name.length == 0) {
        notific_ctor_err_ctrl("empty save name`")
        return
    }

    let pwd_hash = cmp_pwd_hash(form.value.pwd, save_name)
    if (!pwd_hash) { return }

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
        let msg = (props.isSave ? 'сохранено' : 'загружено')
        msg = `успешно ${msg}!`
        notific_ctor_ok_ctrl(msg)
    }).catch(err => {
        var err = err.response.data
        notific_ctor_err_ctrl(`[${err.code}]: ${err.msg}`) 
    });
}

</script>

<template>
    <CtrlLogo />

    <form class="ctrl-form" v-on:submit.prevent="onSubmit">
        <h4 class="ctrl-header" v-html="header" />
        <CtrlPwd :form="form" :line="lineN" />
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