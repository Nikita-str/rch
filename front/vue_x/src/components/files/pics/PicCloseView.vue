<script setup>
import { ref, defineEmits, defineProps } from 'vue'
defineProps({
    img_path: {type: String, default: ""},
})
const emit = defineEmits(['selected'])

let imgRef = ref(null)

function onImgLoad(e) {
    let img = e.target
    console.log("TODO:DEL:ON IMG LOAD", img)
    let real_w = img.naturalWidth
    let real_h = img.naturalHeight
}
</script>

<template>
    <div id="pic-cview-x">
        <div class="pic-close-view">
            <h4 class="pic-cview-h">{{img_path}}</h4>
            <img ref="imgRef" :src="img_path" @load="onImgLoad" :alt="img_path" />
        </div>
    </div>
</template>

<style>
#pic-cview-x {
    display: block;
    position: fixed;
    z-index: 15;
    width: 100%;
    height: 100%;
    background-color: var(--r-col-bg-img-view);
}
.pic-close-view {
    line-height: 1.5em;
    display: inline-block;
    padding: 1.5em;
    padding-top: 0em;
    padding-bottom: 1.2em;
    background: var(--r-col-bg-light-blue-50);
}
.pic-cview-h {
    text-align: center;
    color: var(--r-col-blue);
}
</style>