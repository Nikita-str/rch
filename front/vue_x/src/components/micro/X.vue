<script setup>
import { defineEmits, defineProps, computed } from "vue"
    
const emit = defineEmits(['x'])
function onClose() { emit('x') }

const props = defineProps({
    bottom: {
        type: Boolean,
        default: false,
    },
    lineHeight: {
        type: String,
        default: null,
    },
    paddingRight: {
        type: String,
        default: "0.5em",
    },
    zIndex: {
        type: Number,
        default: null,
    },
})

const xStyle = computed(() => {
    let style_obj = {
        'padding-right': props.paddingRight,
        'z-index': props.zIndex,
    }

    if (props.lineHeight) {
        style_obj['line-height'] = props.lineHeight
    }

    if (props.bottom) {
        style_obj['bottom'] = 0
    } else {
        style_obj['top'] = 0
    }

    return style_obj
})
</script>

<script>
export const X_CLASS_NAME = "x-button"
</script>

<template>
    <div class="nonselectable x-button" :style="xStyle" @click.left.self.prevent="onClose">X</div>
</template>


<style>
.x-button {
    border: none;
    position: absolute;
    right: 0;
    color: var(--r-col-bg);
    background: #0000;
    padding: 2px;
    font-weight: bold;
    cursor: pointer;
}
.x-button:hover { color: var(--r-col-crab-light); }
</style>

