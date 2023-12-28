<script setup>
import CtrlLogo from './CtrlLogo.vue'
import CtrlFormText from './CtrlFormText.vue'
import CtrlFormDone from './CtrlFormDone.vue'
import CtrlOthersList from './CtrlOthersList.vue'
import CtrlPwd from './CtrlPwd.vue'
import { ref } from 'vue'
import { useStore } from 'vuex'
import axios from 'axios'
import { notific_ctor_err_ctrl, notific_ctor_ok_ctrl, cmp_pwd_hash } from '@/js/elems/ctrl.js'
import { trim } from '@/js/fns.js'

const store = useStore()

function newFormContent() {
    return {
        pwd: null,
        tag: null,
        url: null,
        name: null,
        descr: null,
        max_thr_qty: 50,
    }
}
const form = ref(newFormContent())

function onSubmit() {
    let tag = form.value.tag
    if (tag && tag.length == 0) { tag = null }

    let url = form.value.url
    if (!url) { 
        notific_ctor_err_ctrl("emtpty url")
        return
    }
    url = trim(url, '/')
    if (url.length == 0) { 
        notific_ctor_err_ctrl("emtpty url")
        return
    }

    let name = form.value.name
    if (!name || (name.length == 0)) { 
        notific_ctor_err_ctrl("emtpty name")
        return
    }

    let max_thr_qty = parseInt(form.value.max_thr_qty)
    if (!max_thr_qty) { notific_ctor_err_ctrl("emtpty max thr count"); return }
    if (max_thr_qty < 1) { notific_ctor_err_ctrl("max thr count must be more than 0"); return }

    let descr = form.value.descr

    let pwd_hash = cmp_pwd_hash(form.value.pwd, `${url}#${name}`)
    if (!pwd_hash) { return }

    let data = {
        pwd_hash,
        tag,
        url,
        name,
        descr,
        max_thr_qty,
    }
    console.log(data)
    form.value = newFormContent();
    
    axios({
        url: store.getters.getPort + '/~~ctrl~~/add_board',
        method: 'post',
        data,
    }).then(_ => {
        notific_ctor_ok_ctrl('ну, добавили')
    }).catch(err => {
        var err = err.response.data
        notific_ctor_err_ctrl(`[${err.code}]: ${err.msg}`) 
    });
}

</script>

<template>
    <CtrlLogo />
    <form class="ctrl-form" v-on:submit.prevent="onSubmit">
        <h4 class="ctrl-header">ADD BOARD</h4>
        <CtrlPwd :form="form" :line="3" />
        <CtrlFormText v-model="form.tag" :tab="2" placeholder="может быть пустым">tag:</CtrlFormText>
        <CtrlFormText v-model="form.url" :tab="3" :maxLen="16" placeholder="/url/">/url/</CtrlFormText>
        <CtrlFormText v-model="form.name" :tab="4" :maxLen="16" placeholder="а теперь по русски">имя борды:</CtrlFormText>
        <CtrlFormText v-model="form.descr" :tab="5">описаньеце:</CtrlFormText>
        <CtrlFormText v-model="form.max_thr_qty" :tab="6" :maxLen="3" :isNumber="true">макс. тредов:</CtrlFormText>
        <CtrlFormDone />
    </form>
    <br/>
    <CtrlOthersList />
</template>
