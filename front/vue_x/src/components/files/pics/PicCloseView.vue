<script setup>
import { ref, defineEmits, defineProps, computed } from 'vue'

const emit = defineEmits(['close'])

function onClose() {
    console.log("TODO:DEL:IMG VIEW:CLOSE")
    emit('close')
}

const props = defineProps({
    img_path: {type: String, required: true},
    expected_w: {type: Number, default: 0},
    expected_h: {type: Number, default: 0},
})
const PIC_CVIEW_IMG_ID = 'pic-cview-img'


const img_exp_sz = computed(() => {
    if (props.expected_w == 0) return null
    if (props.expected_h == 0) return null
    return {w: props.expected_w, h: props.expected_h} 
})
const img_full_path = computed(() => `/imgs/pp/${props.img_path}`)
const init_w = computed(() => {
    let expected_w = props.expected_w
    let expected_h = props.expected_h

    if (expected_w == 0) return null
    if (expected_h == 0) return null
    return calcWidth(expected_w, expected_h)
})
const init_w_px = computed(() => init_w.value ? `${init_w.value}px` : null)
const inti_style = computed(() => init_w_px.value ? `width: ${init_w_px.value};` : null)

let imgRef = ref(null)
let imgRealSz = ref(null)

function calcWidth(w, h) {
    let coef_w = w / window.innerWidth
    let coef_h = h / window.innerHeight
    let coef = Math.max(coef_w, coef_h)
    const MAX_COEF = 0.85
    if (coef <= MAX_COEF) {
        return w
    } else {
        let coef_x = coef / MAX_COEF
        var w = Math.max(1, Math.floor(w / coef_x))
        return w
    }
}

//async 
function onImgLoad(e) {
    // await new Promise(r => setTimeout(r, 2000))

    // let el = document.getElementById(PIC_CVIEW_IMG_ID)
    
    let img = e.target
    let real_w = img.naturalWidth
    let real_h = img.naturalHeight

    imgRealSz.value = {w: real_w, h: real_h }
    
    let w = calcWidth(real_w, real_h)
    img.style.width = `${w}px`
}
</script>

<template>
    <div tabindex="0" id="pic-cview-x" class="nonselectable" @keyup.esc="onClose" @click.left.self="onClose">
        <div class="centered pic-close-view">
            <h4 class="pic-cview-h">{{img_path}}{{ imgRealSz ? ` | ${imgRealSz.w}x${imgRealSz.h}` : img_exp_sz ? ` | ${img_exp_sz.w}x${img_exp_sz.h}` : '' }}</h4>
            <img :id="PIC_CVIEW_IMG_ID" ref="imgRef" :src="img_full_path" @load="onImgLoad" :alt="img_path" :style="inti_style" />
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