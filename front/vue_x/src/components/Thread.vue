<script setup>
    import PageNotFound from './PageNotFound.vue'
    import PageAwait from './PageAwait.vue'

    import AwaitDots from './micro/awaiters/BigAwaitDots.vue'
    import AwaitText from './micro/awaiters/BigAwaitText.vue'

    import { trim } from '../js/fns'
    import { mapActions } from 'vuex'
    import BoardHeader from './BoardHeader.vue'
    import ThreadView from './ThreadView.vue'
</script>

<script>

    function dataRecalc() {
        return {
            boardExist: null,
            all_loaded: false,
            header: null,
            posts: null,
            err: null,
            next_n: 0,
        }
    }

    export default {
    data() { return dataRecalc() },
    computed: {
        boardUrl() { return trim(this.$route.path, "/").split('/')[0] },
        thrN() { return trim(this.$route.path, "/").split('/')[1] },
    },
    mounted() {
        this.upd()
    },
    watch: {
        '$route' (_to, _) {
            this.upd()
        }
    },
    methods: {
        ...mapActions({ getReq_Board_Name: "getReq_Board_Name", }),
        ...mapActions({ getReq_Thread_ThrLoad: "getReq_Thread_ThrLoad", }),
        upd() {
            this.dataRecalc()
            this.boardInfoLoad()
            this.thrLoad()
        },
        dataRecalc() {
            Object.assign(this.$data, dataRecalc())
        },
        boardInfoLoad() {
            this.boardExist = this.$store.getters.getBoardName(this.boardUrl);
            if (this.boardExist === null) {
                this.getReq_Board_Name(this.boardUrl).then(res => {
                    this.boardExist = res;
                });
            }
        },
        thrLoad() {
            const N_LOAD = 15
            let board_url = this.boardUrl
            let op_post_n = this.thrN
            let from = this.next_n
            let n_load = N_LOAD
            this.getReq_Thread_ThrLoad({board_url, op_post_n, from, n_load}).then(res_x => {
                console.log('[res_x]', res_x)
                console.log('[data]', this.$data)
                
                if (res_x.is_error === true) {
                    this.err = res_x
                    return
                }
                
                let res = res_x.loaded_posts
                let load_len = res.length 
                if (this.posts === null) {
                    this.posts = res
                } else {
                    this.posts = this.posts.concat(res)
                }

                if (res_x.header !== null) {
                    this.header = res_x.header
                }

                this.all_loaded = res_x.all_loaded
                this.next_n = this.next_n + load_len
            });
        },
    },
}
</script>

<template>
    <PageAwait v-if="boardExist === null" 
        :msgFull="'Выяс>>ня<<ем состояние ' + boardUrl + '#' + thrN" 
        back-link-text="выяс>>ня<<ть"
    />
    <div class="board-inner" v-else-if="boardExist.name">
        <BoardHeader :boardName="boardExist.name" :boardUrl="boardUrl" :isCatalog="true" />

        <AwaitDots v-if="posts === null && err === null" />
        <AwaitText v-else-if="err && err.code == 1" :text="'доска /' + boardUrl + '/ не существует?!'" />
        <!-- TODO: доска удалена (получаеца) -->
        <AwaitText v-else-if="err && err.code == 2" :text="'тред не найден...'" />
        <!-- TODO: если посты были -- значит тред удален -->

        <ThreadView v-else :posts="posts" :posts_qty="posts.length" :header="header" :all_loaded="all_loaded" />
        
        <div name="bottom-indent" style="width: 1px; height: 1.2cm;"/>
    </div>
    <PageNotFound v-else /> 
</template>

