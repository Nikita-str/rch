<script setup>
import PostPic from './PostPic.vue'
import { defineEmits } from 'vue'

const emit = defineEmits(['img-click'])
function imgClick(info) { emit('img-click', info) }
</script>

<script>
export default {
    props: {
        imgsInfo: {
            type: Object,
            required: true,
        },
    },
    computed: {
        single() {
            return this.imgsInfo.length == 1
        },
    }
}
</script>

<template>
    <div class="post-pics" :style="{'float': single ? '' : 'none'}">
        <template v-for="imgInfo in imgsInfo">
            <PostPic :imgInfo="imgInfo" :single="single" @img-click="imgClick" />
        </template>
    </div>
</template>

<style scoped>
.post-pics {
    display: flex;
    float: left;
    flex-flow: row wrap;
}
</style>