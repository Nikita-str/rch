<script setup>
import { defineEmits } from 'vue'
const emit = defineEmits(['pic-cancel'])

function onCancel() {
    emit('pic-cancel')
}
</script>

<script>
export default {
    props: {
        file: {
            type: Object,
            required: true,
        },
        maxDimSz: { type: String, default: '6.4em' },
        bottomH: { type: String, default: '0.8em' },
    },
    computed: {
        sizeText() {
            const KB = 1024
            const MB = KB * 1024
            let sz = this.file.size
            if (sz <= KB) { return `1KB` }
            else if (sz < MB) {
                sz = sz / KB
                sz = Math.round(sz * 10) / 10 
                return `${sz}KB`
            } else {
                sz = sz / MB
                sz = Math.round(sz * 10) / 10 
                return `${sz}MB`
            } 
        },
    },
}
</script>

<template>
    <div class="pic-sinlge-view pic-sinlge-view-max-sz">
        <img class="pic-sinlge-view-max-sz" :src="file.url" :alt="file.name" :title="file.name" />
        <div class="pic-sinlge-view-bottom">{{sizeText}}</div>
        <button class="pic-sinlge-view-bottom-x" @click.left.stop.prevent="onCancel">X</button>
    </div>
</template>

<style scoped>
.pic-sinlge-view {
    text-align: center;
    background: var(--r-col-bg-light-blue);
    margin-bottom: 3px;
    margin-right: 3px;
}
.pic-sinlge-view-max-sz {
    max-width: v-bind(maxDimSz);
    max-height: v-bind(maxDimSz);
}
.pic-sinlge-view-bottom {
    position: absolute;
    bottom: 0;
    right: 0;
    background: var(--r-col-blue-80);
    color: var(--r-col-crab-light);
    z-index: 1;
    width: 100%;
    padding: 2px;
    line-height: v-bind(bottomH);
    text-align: start;
    font-weight: bold;
}
.pic-sinlge-view-bottom-x {
    border: none;
    position: absolute;
    bottom: 0;
    right: 0;
    z-index: 2;
    color: var(--r-col-bg-dark);
    background: #0000;
    padding: 2px;
    line-height: v-bind(bottomH);
    width: calc(v-bind(bottomH) * 2);
    text-align: center;
    font-weight: bold;
    cursor: pointer;
}
.pic-sinlge-view-bottom-x:hover {
    color: var(--r-col-crab-light);
}
</style>