
<script setup>
    import ThreadView from './ThreadView.vue'
    // import { trim } from '../js/fns'
    import { mapActions } from 'vuex'

    import AwaitDots from './micro/awaiters/BigAwaitDots.vue'
    import AwaitText from './micro/awaiters/BigAwaitText.vue'
</script> 

<script> 
const THR_CHUNK_LOAD = 10;

function dataRecalc(_new_path) {
    return {
        // boardUrl: trim(new_path, "/").split('/')[0],
        thrs: null,
        thrs_op_n: null,
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
                console.log('[thr load\'ed]', res, this.thrs)
                // TODO: remove duplication ! (by thrs_op_n)
                // TODO: getReq_Board_ThrsLoad : add Set param of known thrs_op_n
            });
        },
    },
    mounted() {
        this.thrLoad()
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
        <ThreadView v-for="thr in thrs" :posts="thr.posts" :posts_qty="thr.posts_qty" :header="thr.header" />
    </template>
</template>
