<script setup>
import ViewSingle from './ViewSingle.vue'

import { ref, defineEmits } from 'vue'
let outer = ref(null)
let em_to_px = ref(null)

const emit = defineEmits(['pic-cancel'])
function onCancel(index) {
    emit('pic-cancel', index)
}
</script>

<script>
const MIN_COLUMNS = 3

export default {
    props: {
        files: {
            type: Array,
            required: true,
        },
    },
    // computed: { }  
    data () {
        return {
            resizeObs: null,
            columns: MIN_COLUMNS,
        }
    },
    mounted () {
        this.resizeObs = new ResizeObserver(this.onResize)
        this.resizeObs.observe(this.$refs.outer)
        this.onResize()
    },
    beforeDestroy () {
        this.resizeObs.unobserve(this.$refs.outer)
    },
    methods: {
        onResize() {
            if (!this.$refs.outer) { return }
            let new_w = this.$refs.outer.clientWidth
            let em = this.$refs.em_to_px.clientWidth
            let single_sz = 6.4 * em + 3
            let n = Math.floor((new_w - 3) / single_sz)
            this.columns = Math.max(n, MIN_COLUMNS)
        },
    }
}
</script>

<template>
    <div ref="outer">
        <div class="pic-view-grid">
            <template v-for="(file, index) in files">
                <ViewSingle :file="file" @pic-cancel="onCancel(index)" />
            </template>
        </div>
    </div>
    <div ref="em_to_px" style="width:1em;height:0px;" />
</template>

<style scoped>
.pic-view-grid {
    display: grid;
    grid-template-columns: repeat(v-bind(columns), min-content);
}
</style>