<script setup>
    import PostingForm from './PostingForm.vue'
    import { ref } from "vue"
    // import vDrag from "v-drag"
</script>


<script>
const MOUSE_LB = 0x1
export const ELEM_ID = "draggable-posting-form" 
var save_pos = null

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
        window.addEventListener("resize", this.onResize);
    },
    unmounted() {
        window.removeEventListener("resize", this.onResize);
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
                if (save_pos) { 
                    save_pos = {
                        w: w,
                        h: h,
                        top: el.offsetTop,
                        left: el.offsetLeft,
                    }
                }
                style.display = "none"
                return
            }

            if (save_pos === null) save_pos = 'once'

            let el_w = el.clientWidth
            let el_h = el.clientHeight
            if (save_pos && save_pos != 'once') {
                var [top, left] = RectToScreen(el_w, el_h, save_pos.top, save_pos.left)
                style.top = top + 'px'
                style.left = left + "px" 
            } else {
                var top = (h - el_h) / 2
                if (top < 0) { top = 0 }

                var left = w - el_w

                style.top = top + 'px'
                style.left = 'calc(' + left + "px - 1cm)"                    
            }
        },

        onMouseDown(e) {
            if((e.buttons & MOUSE_LB) == MOUSE_LB) {
                e.preventDefault();
                document.addEventListener('mousemove', this.onMouseMove)
                document.addEventListener('mouseup', this.onMouseUp)
            }
        },
        
        onMouseUp(e) {
            if((e.buttons & MOUSE_LB) == 0x0) {
                // console.log('remove')
                e.preventDefault();
                document.removeEventListener('mousemove', this.onMouseMove)
                document.removeEventListener('mouseup', this.onMouseUp)
            }
        },
        
        onMouseMove(e) {
            e.preventDefault();
            let el = document.getElementById(ELEM_ID)
            let style = el.style

            let el_w = el.clientWidth
            let el_h = el.clientHeight

            var top = el.offsetTop + e.movementY 
            var left = el.offsetLeft + e.movementX 

            var [top, left] = RectToScreen(el_w, el_h, top, left)

            style.top = top + "px"
            style.left = left + "px"
        },

        onResize(_e) {
            let el = document.getElementById(ELEM_ID)
            if (el) {
                let [top, left] = RectToScreen(el.clientWidth, el.clientHeight, el.offsetTop, el.offsetLeft)
                el.style.top = top + "px"
                el.style.left = left + "px"
            }
        },
    },
}

function RectToScreen(rect_w, rect_h, top, left) {
    let w = window.innerWidth
    let h = window.innerHeight
    let max_top = h - rect_h - 3;
    let max_left = w - rect_w - 3;
    if (top < 3) top = 3
    else if (top > max_top) {
        top = max_top
    }
    
    if (left < 3) left = 3
    else if (left > max_left) {
        left = max_left
    }

    return [top, left]
}

</script>

<template>
    <div :id="ELEM_ID">
        <div  id="dpf-dragger" @mousedown="onMouseDown" />
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
        border: 1px solid var(--r-col-blue); /* var(--r-col-bg-light-blue); */
        background-color: var(--r-col-bg-dark);
    }

    #dpf-dragger {
        cursor: move;
        height: 1.2em;
        margin-top: 3px;
        margin-left: 3px;
        margin-right: 3px;
        background-color: var(--r-col-bg-light-blue);
    }
</style>