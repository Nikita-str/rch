
<script setup>
import DragAndDropField from "../DragAndDropField.vue";
import { ref, computed, defineEmits, defineProps } from 'vue'

const props = defineProps({
    needCompress: { type: Boolean, default: false },
    singleImgMaxSzKB: {type: Number, default: 1024 * 2},
    totalImgMaxSzKB: {type: Number, default: 1024 * 4},
    maxPicQty: {type: Number, default: 4},
})


const emit = defineEmits(['selected', 'rejected'])

const validImgTys = ['image/png', 'image/webp', 'image/jpeg'];
const acceptTys = computed(() => {
    var tys = ""
    validImgTys.forEach((ty) => tys += ty + ',');
    return tys
})

function onSelectedX(files) {
    let total_sz = 0
    let ok_n = 0
    var ok = new Array()
    var reject = new Array()
    files.forEach((file) => {
        let name = file.name

        const ty = file['type']
        if (!validImgTys.includes(ty)) {
            reject.push({name, err: REJECT_TY})
            return
        }

        const sz_kb = (file['size'] / 1024)
        if (sz_kb > props.singleImgMaxSzKB) {
            reject.push({name, err: REJECT_SINGLE_SZ})
            return
        }
        if ((total_sz + sz_kb) > props.totalImgMaxSzKB) {
            reject.push({name, err: REJECT_TOTAL_SZ})
            return
        }

        if (ok_n >= props.maxPicQty) {
            reject.push({name, err: REJECT_PIC_QTY})
            return
        }
        
        if (ok) {
            total_sz += sz_kb
            ok_n += 1
            ok.push(file)
        }
    })
    
    if (reject.length != 0) { emit('rejected', [... reject]) }
    if (ok.length != 0) { emit('selected', [... ok]) }
}
</script>

<script>
export const REJECT_TY = 'BAD_TY'
export const REJECT_PIC_QTY = 'PIC_QTY'
export const REJECT_TOTAL_SZ = 'TOTAL_SZ'
export const REJECT_SINGLE_SZ = 'SINGLE_SZ'
</script>



<template>
    <DragAndDropField whatDragAwait="ПИКЧИ" :accept-tys="acceptTys" @selected="onSelectedX"/>
</template>