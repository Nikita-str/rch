<script setup>
    // defineProps({
    //     msg: {
    //         type: String,
    //         required: true,
    //     },
    //     open_boards: {
    //         type: Number,
    //         required: true,
    //     },
    //     posting_speed: {
    //         type: Number,
    //         required: true,
    //     },
    //     total_post: {
    //         type: Number,
    //         required: true,
    //     },
    // })


function Bnum(x) {
  return `<span style="font-weight:bold;">${x}</span>`
}
</script>

<script>
import BoldX from './micro/BoldX.vue'
export default {
    props: {
        msg: {
            type: String,
            required: true,
        },
        open_boards: {
            type: Number,
            required: true,
        },
        posting_speed: {
            type: Number,
            required: true,
        },
        total_post: {
            type: Number,
            required: true,
        },
    },
    components: {
        BoldX
    }
}
</script>

<template>
    <p> {{ msg }} </p>
    <p style="margin-left: 5%;">Сейчас открыто аж <BoldX :val=open_boards /> досок!</p>
    <!-- <p style="margin-left: 10%;">Скорость постинга всего-то <Bnum posting_speed>{{  }}</Bnum>, поднажмем!</p> -->
    <p style="margin-left: 10%;">Скорость постинга всего-то <BoldX :val=posting_speed />, поднажмем!</p>
    <p style="margin-left: 15%;">За все время было <BoldX :val=total_post /> постов!</p>
</template>