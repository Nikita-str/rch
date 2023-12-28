<script setup>
import CtrlFormText from './CtrlFormText.vue'
import LineN from './CtrlFormLine.vue'
import { defineProps } from 'vue'

const props = defineProps({
    form: Object,
    tab: {
        type: Number,
        default: 1,
    },
    line: Number,
})
</script>

<template>
    <CtrlFormText v-model="form.pwd" :tab="tab" :maxLen="8 + 1 + 12" placeholder="<nonce><space><pwd>">
            nonce&amp;pwd<LineN :n="line" />:
    </CtrlFormText>
</template>
