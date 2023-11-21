<script setup>
import DragAndDropField from "../DragAndDropField.vue";
import { ref, computed, defineEmits, defineProps } from 'vue'

defineProps({
    needCompress: { type: Boolean, default: false },
    acceptTys: {type: String, default: ""},
})

const emit = defineEmits(['selected', 'rejected'])

const validImgTys = ['image/png', 'image/webp', 'image/jpeg'];
const acceptTys = computed(() => {
    var tys = ""
    validImgTys.forEach((ty) => tys += ty + ',');
    return tys
})

function onSelectedX(files) {
    var ok = new Array()
    var reject = new Array()
    files.forEach((file) => {
        const ty = file['type']
        if (validImgTys.includes(ty)) {
            ok.push(file)
        } else {
            reject.push(file)
        }
    })
    
    if (reject.length != 0) { emit('rejected', [... reject]) }
    if (ok.length != 0) { emit('selected', [... ok]) }
}
</script>



<template>
    <DragAndDropField :accept-tys="acceptTys" @selected="onSelectedX"/>
</template>