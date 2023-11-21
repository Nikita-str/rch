<script setup>
import { ref, defineEmits, defineProps } from 'vue'
defineProps({
    needCompress: { type: Boolean, default: false },
})
const emit = defineEmits(['selected'])

const validImgTys = ['image/png', 'image/webp', 'image/jpeg']; // validFileTys as props

let fileInput = ref(null)

let active = ref(false)
function setActive() {
    active.value = true
}
function setInactive() {
    active.value = false
}

function callSelected(files) {
    emit('selected', [... files])
}

function onDrop(e) {
    setInactive()
    callSelected(e.dataTransfer.files)
}


function onClickSelect(e) {
    callSelected(e.target.files)
}
</script>


<template>
    <div 
    class="file-dnd-field-std"
    :class="{'file-dnd-field-act': active, 'file-dnd-field-unact': !active}"
    @dragenter.prevent="setActive" @dragover.prevent="setActive" 
    @dragleave.prevent="setInactive"
    @click="$refs.fileInput.click()"
    @drop.prevent="onDrop">
        <span class="file-dnd-field-text">ПЕРЕМЕСТИ ПИКЧИ<br/>CTRL+V</span>
    </div>
    <input ref="fileInput" type="file" style="display: none;" multiple="" accept="image/png, image/webp, image/jpeg" @change="onClickSelect">
</template>


<style scoped>
.file-dnd-field-std {
    width: 100%;
    height: 3.2em; /* 3.6em  OR  2.4em */
    border: dashed 3px;
    text-align: center;
    vertical-align: middle;
    margin-bottom: 3px;
}
.file-dnd-field-act {
    border-color: var(--r-col-crab-light);
    color: var(--r-col-crab-light);
}
.file-dnd-field-unact {
    border-color: var(--r-col-bg-light-blue);
    color: var(--r-col-bg-light-blue);
}

.file-dnd-field-text {
    width: 100%;
    font-weight: bold;
    font-size: 1em; /* 0.8 */
    line-height: 1;

    margin: 0;
    position: absolute;
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%);
}
</style>