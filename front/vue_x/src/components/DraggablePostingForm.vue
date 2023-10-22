<script setup>
    import PostingForm from './PostingForm.vue'
    import { ref } from "vue"
    // import vDrag from "v-drag"
</script>


<script>
export const ELEM_ID = "draggable-posting-form" 

export default {
    props: {
        boardUrl: {
            type: String,
            required: true,
        },
        isNewThr: {
            type: Boolean,
            required: true,
        },
        visible: {
            type: Boolean,
            default: true,
        },
    },
    watch: {
        'visible' (to, from) {
            if (to != from) this.upd()
        }
    },
    mounted() {
        this.upd()
    },
    methods: {
        upd() {
            let w = window.innerWidth
            let h = window.innerHeight

            let el = document.getElementById(ELEM_ID)
            let style = el.style
            
            if (this.visible) {
                style.display = "block"
            } else {
                style.display = "none"
                return
            }
            
            let el_w = el.clientWidth
            let el_h = el.clientHeight

            
            var top = (h - el_h) / 2
            if (top < 0) { top = 0 }

            var left = w - el_w

            console.log('wh', el_w, el_h)

            style.top = top + 'px'
            style.left = 'calc(' + left + "px - 1cm)"
        },

        dragEnd(obj) {
            // console.log('drag-end', obj)
            let el = obj.target
            
            let w = window.innerWidth
            let h = window.innerHeight
            let el_w = el.clientWidth
            let el_h = el.clientHeight
            let max_top = h - el_h - 3;
            let max_left = w - el_w - 3;

            if (el.offsetTop < 3) {
                el.style.top = "3px"
            }
            if (el.offsetLeft < 3) {
                el.style.left = "3px"
            }
            if (el.offsetTop > max_top) {
                el.style.top = max_top + "px"
            }
            if (el.offsetLeft > max_left) {
                el.style.left = max_left + "px"
            }
        },
    },
}

</script>

<template>
    <div :id="ELEM_ID" v-drag="{ handle: '#dpf-dragger' }" @v-drag-end="dragEnd">
        <div  id="dpf-dragger" />
        <!-- <div  style="width: 100%; height: 20px; background-color: crimson;" @click.left="xxx" /> -->
        <div style="position: relative;">
            <PostingForm :boardUrl="boardUrl" :isNewThr="isNewThr" />
        </div>
    </div>
</template>


<style scoped>
    #draggable-posting-form {
        z-index: 3;
        position: fixed;
        /* top: v-bind(); */
        /* left: v-bind(); ... need to be mounted*/
        top: 20vh;
        border: 1px solid var(--r-col-bg-light-blue);
        background-color: var(--r-col-bg-dark);
    }

    #dpf-dragger {
        height: 1.2em;
        margin-top: 3px;
        margin-left: 3px;
        margin-right: 3px;
        background-color: var(--r-col-bg-light-blue);
    }
</style>