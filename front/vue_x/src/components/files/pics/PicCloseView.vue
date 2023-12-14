<script setup>
import X from '../../micro/X.vue';
import { X_CLASS_NAME } from '../../micro/X.vue';
import { ref, defineEmits, defineProps, computed } from 'vue'

const emit = defineEmits(['close'])
function onClose() { emit('close') }

const props = defineProps({
    name: {type: String, default: ""},
    img_path: {type: String, required: true},
    expected_w: {type: Number, default: 0},
    expected_h: {type: Number, default: 0},
})
const PIC_CVIEW_ID = 'pic-cview'
const MOUSE_LB = 0x1
const PX = 'px'

const img_name_full = computed(() => {
    return (props.name.length == 0) ? props.img_path : props.name
})
const img_name = computed(() => {
    let name = img_name_full.value
    const MAX_LEN = 24
    const HALF = (MAX_LEN >> 1) - 1
    if (name.length > MAX_LEN) {
        return name.substring(0, HALF) + "[..]" + name.slice(-HALF)
    } else {
        return name
    }
})
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

let scaleCoef = ref(null)
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
    scaleCoef.value = w / real_w
    img.style.width = `${w}px`
}

function getElImg() {
    let el = document.getElementById(PIC_CVIEW_ID)
    let img = el.getElementsByTagName('img')[0]
    return {el, img}
}

function onMouseDown(e) {
    e.preventDefault();

    if (e.target.classList.contains(X_CLASS_NAME)) return

    let {el, img} = getElImg()
    img.style.cursor = 'move'

    el.style.left = el.offsetLeft + PX
    el.style.top = el.offsetTop + PX
    
    document.addEventListener('mousemove', onMouseMove)
    document.addEventListener('mouseup', onMouseUp)
}
        
function onMouseUp(e) {
    if((e.buttons & MOUSE_LB) == 0x0) {
        e.preventDefault();
        
        let {_, img} = getElImg()
        img.style.cursor = 'default'

        document.removeEventListener('mousemove', onMouseMove)
        document.removeEventListener('mouseup', onMouseUp)
    }
}

function onMouseMove(e) {
    let el = document.getElementById(PIC_CVIEW_ID)

    el.style.left = el.offsetLeft + e.movementX + PX
    el.style.top = el.offsetTop + e.movementY + PX
}

function onWheel(e) {
    if (e.deltaY == 0) return
    let scale_up = e.deltaY < 0

    let cur_coef = scaleCoef.value
    if (!cur_coef) return
    
    const EPS = 0.0001
    const VALID_COEFS = [0.1, 0.125, 0.15, 0.2, 0.25, 0.3, 0.4, 0.5, 0.6, 0.75, 0.8, 1.0, 1.25, 1.5, 2.0, 4.0, 8.0, 16.0, 32.0]
    let valid_coef_index = 0
    for (const valid_c of VALID_COEFS) {
        if (cur_coef <= valid_c + EPS) break
        valid_coef_index += 1
    }

    if (scale_up && valid_coef_index == VALID_COEFS.length) return
    if (!scale_up && valid_coef_index == 0) return

    let prev_coef = VALID_COEFS[valid_coef_index]

    let coef_index_next = null;
    if (scale_up) {
        let need_to_cur_scale = (prev_coef - cur_coef) > EPS

        coef_index_next = need_to_cur_scale ? valid_coef_index : valid_coef_index + 1
        if (coef_index_next == VALID_COEFS.length) return
    } else {
        coef_index_next = valid_coef_index - 1
    }

    let new_coef = VALID_COEFS[coef_index_next]
    var w = imgRealSz.value.w
    var w = Math.max(1, Math.floor(w * new_coef))
    scaleCoef.value = new_coef
    let {el, img} = getElImg()
    if (new_coef >= 4.0 - EPS) {
        img.style.imageRendering = 'pixelated';
    } else {
        img.style.imageRendering = 'auto';
    }

    let el_rect = el.getBoundingClientRect();
    let scale = new_coef / prev_coef
    var dx = e.clientX - el_rect.left;
    var dy = e.clientY - el_rect.top;
    let prev_w = el_rect.width;
    let prev_h = el_rect.height;
    let cx = prev_w / 2;
    let cy = prev_h / 2;
    let c_dx = dx - cx;
    let c_dy = dy - cy;
    let c_dx_scaled = c_dx * scale;
    let c_dy_scaled = c_dy * scale;

    el.style.left = (el.offsetLeft + (c_dx - c_dx_scaled)) + PX
    el.style.top = (el.offsetTop + (c_dy - c_dy_scaled)) + PX

    img.style.width = `${w}px`
}
</script>

<template>
    <div tabindex="0" id="pic-cview-outer" class="nonselectable" @keyup.esc="onClose" @click.left.self="onClose">
        <div :id="PIC_CVIEW_ID" class="centered pic-close-view" @mousedown.left.prevent="onMouseDown" @wheel.prevent="onWheel">
            <h4 class="pic-cview-h" :title="img_name_full">{{img_name}}{{ imgRealSz ? ` | ${imgRealSz.w}x${imgRealSz.h}` : img_exp_sz ? ` | ${img_exp_sz.w}x${img_exp_sz.h}` : '' }}</h4>
            <X :lineHeight="'1.3'" @x="onClose" />
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
    left: 0;
    top: 0;
    background-color: var(--r-col-bg-img-view);
}
.pic-close-view {
    text-align: center;
    line-height: 1.5em;
    display: inline-block;
    padding: 0.5em;
    padding-top: 0em;
    padding-bottom: 0.3em;
    /* background: var(--r-col-bg-light-blue-50); */
    background: var(--r-col-img-view-border);
    cursor: move;
}
.pic-cview-h {
    color: var(--r-col-blue);
    padding-left: 1.5em;
    padding-right: 1.5em;
}
.pic-cview-img {
    cursor: default;
}
</style>