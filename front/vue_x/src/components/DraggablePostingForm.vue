<script setup>
    import PostingForm from './PostingForm.vue'
    import { ref } from "vue"
    // import vDrag from "v-drag"
</script>


<script>
const MOUSE_LB = 0x1
export const ELEM_ID = "draggable-posting-form" 
var save_pos = null
var drag_delta = null
const INDENT = 3

export default {
    props: {
        boardUrl: {
            type: String,
            required: true,
        },
        /** if null => isNewThr */
        opPostN: {
            type: Number,
            required: true,
        },
        visible: {
            type: Boolean,
            default: true,
        },
    },
    watch: {
        'visible' (to, from) {
            if (to != from) this.upd(false)
        }
    },
    mounted() {
        this.upd(true)
        window.addEventListener("resize", this.onResize);
    },
    unmounted() {
        window.removeEventListener("resize", this.onResize);
    },
    methods: {
        upd(called_by_mounted) {
            let w = window.innerWidth
            let h = window.innerHeight

            let el = document.getElementById(ELEM_ID)
            let style = el.style
            
            if (this.visible) {
                style.display = "block"
            } else {
                if (!called_by_mounted && save_pos) { 
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
                let el = document.getElementById(ELEM_ID)

                let el_rect = el.getBoundingClientRect()
                let drager_rect = e.target.getBoundingClientRect()
                drag_delta = {
                    dx: Math.round(drager_rect.left - el_rect.left) + e.layerX,
                    dy: Math.round(drager_rect.top - el_rect.top) + e.layerY,
                }

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

            // var dx = e.movementX
            // var dy = e.movementY
            var dx = e.pageX - (el.offsetLeft + drag_delta.dx)
            var dy = e.pageY - (el.offsetTop + drag_delta.dy)

            var top = el.offsetTop + dy
            var left = el.offsetLeft + dx 

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
    let max_top = h - rect_h - INDENT;
    let max_left = w - rect_w - INDENT;
    if (top < INDENT) top = INDENT
    else if (top > max_top) {
        top = max_top
    }
    
    if (left < INDENT) left = INDENT
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
            <PostingForm :boardUrl="boardUrl" :opPostN="opPostN" />
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