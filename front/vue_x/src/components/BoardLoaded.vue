
<script setup>
    import ThreadView from './ThreadView.vue'
    // import { trim } from '../js/fns'
    import { mapActions } from 'vuex'

    import AwaitDots from './micro/awaiters/BigAwaitDots.vue'
    import AwaitText from './micro/awaiters/BigAwaitText.vue'

    import { vElementVisibility } from '@vueuse/components'
</script> 

<script> 
const THR_LAST_N_UPD = 2
const THR_CHUNK_LOAD = 10 // 2 FOR TESTS
const THR_AUTO_UPD_MS = 60_000 // 5_000

function dataRecalc(_new_path) {
    return {
        // boardUrl: trim(new_path, "/").split('/')[0],
        thrs: null,
        thrs_op_n: null,
        cur_load_more: false,
        auto_upd_timer: null,
        visible_mask: 0,
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
            if (this.cur_load_more) { return }
            this.cur_load_more = true
            let board_url = this.boardUrl;
            let from = (this.thrs === null) ? 0 : this.thrs.length;
            let to = from + THR_CHUNK_LOAD;

            this.getReq_Board_ThrsLoad({board_url, from, to}).then(res_x => {
                let res = res_x.thrs
                if (this.thrs === null) {
                    this.thrs = res
                } else {
                    this.thrs = this.thrs.concat(res)
                }
                this.cur_load_more = false

                if ((this.auto_upd_timer === null) && ((this.thrs.length == 0) || ((res.length == 0) && (this.visible_mask > 0)))) {
                    // console.log('SET TIMER');
                    this.auto_upd_timer = setTimeout(() => {
                        // console.log('AUTO');
                        this.removeAutoUpdTimer() 
                        this.thrLoad()
                    }, THR_AUTO_UPD_MS)
                }
                console.log('[thr load\'ed]', res, this.thrs)
                // TODO: remove duplication ! (by thrs_op_n)
                // TODO: getReq_Board_ThrsLoad : add Set param of known thrs_op_n
            });
        },               
        onElementVisibility(visible_n, visible) {
            if (visible) {
                this.visible_mask |= visible_n
            } else {
                this.visible_mask &= ~visible_n
            }
            if ((this.visible_mask == 0) && (this.auto_upd_timer !== null)) {
                this.removeAutoUpdTimer()
            }
            if (visible) {
                this.thrLoad()
            }
        },
        removeAutoUpdTimer() {
            if (this.auto_upd_timer !== null) {
                // console.log('REMOVE TIMER');
                clearInterval(this.auto_upd_timer)
                this.auto_upd_timer = null
          }
        }
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
            <ThreadView :posts="thr.posts" :posts_qty="thr.posts_qty" :header="thr.header" v-if="index + THR_LAST_N_UPD < thrs.length"/>
            <ThreadView :posts="thr.posts" :posts_qty="thr.posts_qty" :header="thr.header" v-else
                    v-element-visibility="(vis) => onElementVisibility(1 << (thrs.length - index - 1), vis)"
            />
        </template>

    </template>
    <AwaitDots v-if="thrs !== null && cur_load_more" />
</template>
