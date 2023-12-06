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
const PIC_CVIEW_ID = 'pic-cview'
const MOUSE_LB = 0x1
const PX = 'px'

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
    
    let img = e.target
    let real_w = img.naturalWidth
    let real_h = img.naturalHeight

    imgRealSz.value = {w: real_w, h: real_h }
    
    let w = calcWidth(real_w, real_h)
    img.style.width = `${w}px`
}

function onMouseDown(e) {
    e.preventDefault();

    if (e.target.classList.contains("pic-cview-x")) return

    let el = document.getElementById(PIC_CVIEW_ID)
    let img = el.getElementsByTagName('img')[0]
    img.style.cursor = 'move'

    el.style.left = el.offsetLeft + PX
    el.style.top = el.offsetTop + PX
    
    document.addEventListener('mousemove', onMouseMove)
    document.addEventListener('mouseup', onMouseUp)
}
        
function onMouseUp(e) {
    if((e.buttons & MOUSE_LB) == 0x0) {
        e.preventDefault();
        
        let el = document.getElementById(PIC_CVIEW_ID)
        let img = el.getElementsByTagName('img')[0]
        img.style.cursor = 'default'

        document.removeEventListener('mousemove', onMouseMove)
        document.removeEventListener('mouseup', onMouseUp)
    }
}

function onMouseMove(e) {
    let el = document.getElementById(PIC_CVIEW_ID)

    el.style.left = el.offsetLeft + e.movementX + PX
    el.style.top = el.offsetTop+ e.movementY + PX
}
</script>

<template>
    <div tabindex="0" id="pic-cview-outer" class="nonselectable" @keyup.esc="onClose" @click.left.self="onClose">
        <div :id="PIC_CVIEW_ID" class="centered pic-close-view" @mousedown.left.prevent="onMouseDown">
            <h4 class="pic-cview-h">{{img_path}}{{ imgRealSz ? ` | ${imgRealSz.w}x${imgRealSz.h}` : img_exp_sz ? ` | ${img_exp_sz.w}x${img_exp_sz.h}` : '' }}</h4>
            <h4 class="pic-cview-x" @click.left.self.prevent="onClose">X</h4>
            <img ref="imgRef" class="pic-cview-img" :src="img_full_path" @load="onImgLoad" :alt="img_path" :style="inti_style" />
        </div>
    </div>
</template>

<style>
#pic-cview-outer {
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
    padding: 0.5em;
    padding-top: 0em;
    padding-bottom: 0.3em;
    background: var(--r-col-bg-light-blue-50);
    cursor: move;
}
.pic-cview-h {
    text-align: center;
    color: var(--r-col-blue);
}
.pic-cview-x {
    border: none;
    position: absolute;
    right: 0;
    top: 0;
    line-height: 1.3;
    color: var(--r-col-bg-dark);
    background: #0000;
    padding: 2px;
    font-weight: bold;
    cursor: pointer;
    padding-right: 0.5em;
}
.pic-cview-x:hover {
    color: var(--r-col-crab-light);
}
</style>