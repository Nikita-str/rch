
<script setup>
    import ThreadView from './ThreadView.vue'
    // import { trim } from '../js/fns'
    import { mapActions } from 'vuex'

    import AwaitDots from './micro/awaiters/BigAwaitDots.vue'
    import AwaitText from './micro/awaiters/BigAwaitText.vue'

    import { vElementVisibility } from '@vueuse/components'
    
    import PicCloseView from './files/pics/PicCloseView.vue'
</script> 

<script> 
const THR_LAST_N_UPD = 3
const THR_CHUNK_LOAD = 10 // 2 FOR TESTS
const THR_AUTO_UPD_MS = 60_000 // 5_000

function dataRecalc(_new_path) {
    return {
        // boardUrl: trim(new_path, "/").split('/')[0],
        thrs: null,
        cur_load_more: false,
        auto_upd_timer: null,
        last_visible: false,
        all_loaded: false,
        last_loaded_time: new Date(),
        
        picCloseViewInfo: new Object(),
        picCloseViewVisible: false,
    }
}

export default {
    props: {
        boardUrl: {
            type: String,
            required: true,
        },
    },
    data() { return dataRecalc(this.boardUrl) },
    methods: {
        dataRecalc(new_boardUrl) {
            Object.assign(this.$data, dataRecalc(new_boardUrl))
        },

        ...mapActions({ getReq_Board_ThrsLoad: "getReq_Board_ThrsLoad", }),
        thrLoad() {
            if (this.all_loaded) {
                let cur = new Date()
                let delta_ms = cur - this.last_loaded_time
                if (delta_ms < (THR_AUTO_UPD_MS * 0.8)) {
                    this.addAutoUpdTimer(Math.min(THR_AUTO_UPD_MS, THR_AUTO_UPD_MS * 1.1 - Math.abs(delta_ms)))
                    return
                }
            }
            if (this.cur_load_more) { return }
            this.cur_load_more = true

            let board_url = this.boardUrl;
            let load_n = THR_CHUNK_LOAD;
            let known_n = new Array() // new Set() but then we must convert it to Array
            if (this.thrs) {
                this.thrs.forEach((x) => known_n.push(x.posts[0].n))
            }

            this.getReq_Board_ThrsLoad({board_url, known_n, load_n}).then(res_x => {
                this.all_loaded = res_x.all_loaded
                if (this.all_loaded) {
                    this.last_loaded_time = new Date()
                }

                let thrs = res_x.thrs
                if (this.thrs === null) {
                    this.thrs = thrs
                } else {
                    this.thrs = this.thrs.concat(thrs)
                }
                this.cur_load_more = false

                if ((this.auto_upd_timer === null) && ((this.thrs.length == 0) || (this.all_loaded && this.last_visible))) {
                    this.addAutoUpdTimer(THR_AUTO_UPD_MS)
                }
                console.log('[thr load\'ed]', thrs, this.thrs)
            });
        },               
        onElementVisibility(visible) {
            if ((!this.last_visible) && (this.auto_upd_timer !== null)) {
                this.removeAutoUpdTimer()
            }
            if (visible) {
                this.thrLoad()
            }
        },
        addAutoUpdTimer(t) {
            if (this.auto_upd_timer === null) {
                // console.log('SET TIMER');
                this.auto_upd_timer = setTimeout(() => {
                    // console.log('AUTO');
                    this.removeAutoUpdTimer() 
                    this.thrLoad()
                }, t)
            }
        },
        removeAutoUpdTimer() {
            if (this.auto_upd_timer !== null) {
                // console.log('REMOVE TIMER');
                clearInterval(this.auto_upd_timer)
                this.auto_upd_timer = null
            }
        },
        onPostImgClick(info) {
            this.picCloseViewInfo = info
            this.picCloseViewVisible = true
        },
        onPicCloseViewClose() {
            this.picCloseViewVisible = false
        },
    },
    mounted() {
        this.thrLoad()
    },
    unmounted() {
        this.removeAutoUpdTimer()
    },
    watch: {
        boardUrl(new_boardUrl, _) {
            this.dataRecalc(new_boardUrl)
            this.thrLoad()
        }
    },
}
</script> 

<template>
    <AwaitDots v-if="/* true || */ thrs === null" />
    <AwaitText v-else-if="thrs.length == 0" text="с доски украли все треды!!!" />
    <template v-else>
        <!-- <ThreadView v-for="thr in thrs" :posts="thr.posts" :posts_qty="thr.posts_qty" :header="thr.header" /> -->

        <template v-for="(thr, index) in thrs" >
            <ThreadView :posts="thr.posts" :posts_qty="thr.posts_qty" :header="thr.header" @img-click="onPostImgClick" v-if="index + THR_LAST_N_UPD < thrs.length"/>
            <ThreadView :posts="thr.posts" :posts_qty="thr.posts_qty" :header="thr.header" v-else
                v-element-visibility="(vis) => { 
                    if (index == thrs.length - 1) { last_visible = vis } 
                    onElementVisibility(vis)
                }"
                @img-click="onPostImgClick"
            />
        </template>

    </template>
    <AwaitDots v-if="thrs !== null && cur_load_more" />

    <PicCloseView v-if="picCloseViewVisible" @close="onPicCloseViewClose" 
        :name="picCloseViewInfo.name"
        :img_path="picCloseViewInfo.img_path"
        :expected_w="picCloseViewInfo.expected_w"
        :expected_h="picCloseViewInfo.expected_h"
    />
</template>
