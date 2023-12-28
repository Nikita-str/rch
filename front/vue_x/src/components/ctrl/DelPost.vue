<script setup>
import CtrlLogo from './CtrlLogo.vue'
import CtrlFormText from './CtrlFormText.vue'
import CtrlFormDone from './CtrlFormDone.vue'
import CtrlOthersList from './CtrlOthersList.vue'
import CtrlPwd from './CtrlPwd.vue'
import { ref } from 'vue'
import { useStore } from 'vuex'
import axios from 'axios'
import { notific_ctor_err_ctrl, notific_ctor_ok_ctrl, cmp_pwd_hash, url_prepare, positive_num_prepare } from '@/js/elems/ctrl.js'

const store = useStore()

function newFormContent() {
    return {
        pwd: null,
        url: null,
        op_post_n: null,
        post_n: null,
    }
}
const form = ref(newFormContent())

function onSubmit() {
    let tag = form.value.tag
    if (tag && tag.length == 0) { tag = null }

    let url = url_prepare(form.value.url)
    if (!url) { return }

    let op_post_n = positive_num_prepare(form.value.op_post_n, "OP post N")
    if (!op_post_n) { return }

    let post_n = positive_num_prepare(form.value.post_n, "post N")
    if (!post_n) { return }

    let pwd_hash = cmp_pwd_hash(form.value.pwd, `${url}#${post_n}`)
    if (!pwd_hash) { return }

    let data = {
        pwd_hash,
        board_url: url,
        op_post_n,
        post_n,
    }
    form.value = newFormContent();
    
    axios({
        url: store.getters.getPort + '/~~ctrl~~/del_post',
        method: 'delete',
        data,
    }).then(_ => {
        //TODO:MAYBE: 'долгожданное завершение какого-либо длительного, затруднительного или неприятного процесса, а в данном случае -- конец существования указанного поста'
        notific_ctor_ok_ctrl('пиздец посту')
    }).catch(err => {
        var err = err.response.data
        notific_ctor_err_ctrl(`[${err.code}]: ${err.msg}`) 
    });
}

</script>

<template>
    <CtrlLogo />
    <form class="ctrl-form" v-on:submit.prevent="onSubmit">
        <h4 class="ctrl-header">DEL POST</h4>
        <CtrlPwd :form="form" :line="4" />
        <CtrlFormText v-model="form.url" :tab="2" :maxLen="16" placeholder="/url/">/url/</CtrlFormText>
        <CtrlFormText v-model="form.op_post_n" :tab="3" :maxLen="20" :isNumber="true">OP пост N:</CtrlFormText>
        <CtrlFormText v-model="form.post_n" :tab="4" :maxLen="20" :isNumber="true">пост N:</CtrlFormText>
        <CtrlFormDone />
    </form>
    <br/>
    <CtrlOthersList />
</template>
