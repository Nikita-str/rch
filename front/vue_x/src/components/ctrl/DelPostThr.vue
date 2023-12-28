<script setup>
import CtrlLogo from './CtrlLogo.vue'
import CtrlFormText from './CtrlFormText.vue'
import CtrlFormDone from './CtrlFormDone.vue'
import CtrlOthersList from './CtrlOthersList.vue'
import CtrlPwd from './CtrlPwd.vue'
import { ref, defineProps, computed } from 'vue'
import { useStore } from 'vuex'
import axios from 'axios'
import { notific_ctor_err_ctrl, notific_ctor_ok_ctrl, cmp_pwd_hash, url_prepare, positive_num_prepare } from '@/js/elems/ctrl.js'

const store = useStore()
const props = defineProps({
    isPost: {type: Boolean, required: true},
})

const header = computed(() => {
    return (props.isPost) ? "DEL POST" : "DEL THR"
})
const lineN = computed(() => {
    return (props.isPost) ? 4 : 5
})
const reqUrl = computed(() => {
    return store.getters.getPort + '/~~ctrl~~/' + ((props.isPost) ? 'del_post' : 'del_thr')
})

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

    let post_n = null
    if (props.isPost) {
        post_n = positive_num_prepare(form.value.post_n, "post N")
        if (!post_n) { return }
    }

    let hash_inner_part = (props.isPost) ? `${url}#${post_n}` : `${url}#${op_post_n}`
    let pwd_hash = cmp_pwd_hash(form.value.pwd, hash_inner_part)
    if (!pwd_hash) { return }

    let data = {
        pwd_hash,
        board_url: url,
        op_post_n,
        post_n,
    }
    form.value = newFormContent();
    
    axios({
        url: reqUrl.value,
        method: 'delete',
        data,
    }).then(_ => {
        //TODO:MAYBE: 'долгожданное завершение какого-либо длительного, затруднительного или неприятного процесса, а в данном случае -- конец существования указанного [поста/треда]'
        let msg = 'пиздец ' + ((props.isPost) ? 'посту' : 'треду')
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
        <CtrlFormText v-model="form.url" :tab="2" :maxLen="16" placeholder="/url/">/url/</CtrlFormText>
        <CtrlFormText v-model="form.op_post_n" :tab="3" :maxLen="20" :isNumber="true">OP пост N:</CtrlFormText>
        <CtrlFormText v-if="isPost" v-model="form.post_n" :tab="4" :maxLen="20" :isNumber="true">пост N:</CtrlFormText>
        <CtrlFormDone />
    </form>
    <br/>
    <CtrlOthersList />
</template>
