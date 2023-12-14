<script setup>
import X from '../../micro/X.vue';
import { defineEmits } from 'vue'
const emit = defineEmits(['pic-cancel'])

function onCancel() {
    emit('pic-cancel')
}

const MIN_DIM_SZ = '10px'
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
        <X :zIndex="2" :bottom="true" :paddingRight="'4px'" :lineHeight="bottomH" @x="onCancel" />
        <div class="pic-sinlge-view-spoiler">
            <input
            type="checkbox"
            class="x-checker nonselectable pic-sinlge-view-spoiler-checkbox"
            title="SPOILER!ALERT"
            v-model="file.spoiler"
            style="border-color: var(--r-col-bg-dark);"
            />
        </div>
        <p v-if="file.spoiler" class="centered pic-spoiler-text" style="transform: translate(-50%,-50%) rotate(20deg); font-size: 0.8em;">!SPOILER!</p>
    </div>
</template>

<style scoped>
.pic-sinlge-view {
    text-align: center;
    background: var(--r-col-bg-light-blue);
    margin-bottom: 3px;
    margin-right: 3px;
    width: v-bind(maxDimSz);
    height: v-bind(maxDimSz);
}
.pic-sinlge-view-max-sz {
    min-width: v-bind(MIN_DIM_SZ);
    min-height: v-bind(MIN_DIM_SZ);
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
.pic-sinlge-view-spoiler {
    display: flex;
    position: absolute;
    top: 0;
    right: 0;
    z-index: 3;
    background: var(--r-col-blue-80);
    padding: 3px;
}
.pic-sinlge-view-spoiler-checkbox:checked {
    background-color: var(--r-col-bg-dark);
}
</style>