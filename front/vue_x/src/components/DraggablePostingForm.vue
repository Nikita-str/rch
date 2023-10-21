<script setup>
    import PostingForm from './PostingForm.vue'
    import { ref } from "vue"
    // import vDrag from "v-drag"
</script>


<script> 
export default {
    mounted() {
        this.upd()
    },
    methods: {
        upd() {
            let w = window.innerWidth
            let h = window.innerHeight

            let el = document.getElementById('draggable-posting-form')
            let el_w = el.clientWidth
            let el_h = el.clientHeight

            let style = el.style
            
            var top = (h - el_h) / 2
            if (top < 0) { top = 0 }

            var left = w - el_w

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
    <div id="draggable-posting-form" v-drag="{ handle: '#dpf-dragger' }" @v-drag-end="dragEnd">
        <div  id="dpf-dragger" />
        <!-- <div  style="width: 100%; height: 20px; background-color: crimson;" @click.left="xxx" /> -->
        <div style="position: relative;">
            <PostingForm boardUrl="TODO" :isNewThr=true />
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