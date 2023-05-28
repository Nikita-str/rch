
<script setup>
    import BoardHeader from './BoardHeader.vue'
    import ThreadView from './ThreadView.vue'
    import { trim } from '../js/fns'
    import { mapActions } from 'vuex'
</script> 

<script> 
const THR_CHUNK_LOAD = 10;

function dataRecalc(new_path) {
    return {
        boardUrl: trim(new_path, "/").split('/')[0],
        thrs: null,
        thrs_op_n: null,
    }
}

export default {
    props: {
        boardName: {
            type: String,
            required: true,
        }
    },
    data() { return dataRecalc(this.$route.path) },
    methods: {
        dataRecalc(new_path) {
            Object.assign(this.$data, dataRecalc(new_path))
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
        '$route' (to, _) {
            this.dataRecalc(to.path)
            this.thrLoad()
        }
    },
}
</script> 

<template>
    <div class="board-inner">
        <BoardHeader :boardName="boardName" />

        <div v-if="/* true || */ thrs === null" class="board-sad-text">
            <span class="board-await" style="animation-delay: 0s;">.</span>
            <span class="board-await" style="animation-delay: 1s;">.</span>
            <span class="board-await" style="animation-delay: 2s;">.</span>
        </div>
        <div v-else-if="thrs.length == 0" class="board-sad-text">с доски украли все треды!!!</div>
        <template v-else>
            <ThreadView v-for="thr in thrs" :posts="thr" />
        </template>
    </div>
</template>


<style scoped>
.board-inner {
    width: 100vw;
}

.board-sad-text {
    text-align: center;
    font-size: 1.9em;
    padding-top: 0.35em;
    color: var(--r-col-blue);
    font-weight: 900;
}



@keyframes animx-await-change-color {
  from { color: var(--r-col-blue); }
  20%   { color: var(--r-col-transparent-dbg); }
  40%  { color: var(--r-col-transparent-dbg); }
  60%  { color: var(--r-col-blue); }
  to   { }
}
.board-await {
    animation-name: animx-await-change-color;
    animation-duration: 4s;
    animation-iteration-count: infinite;
    animation-direction: normal;
}
</style>
