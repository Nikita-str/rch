<script setup>
    import PageNotFound from './PageNotFound.vue'
    import PageAwait from './PageAwait.vue'
    import BoardLoaded from './BoardLoaded.vue'
    import { trim } from '../js/fns'
    import { mapActions } from 'vuex'
    import BoardHeader from './BoardHeader.vue'
    import Catalog from './Catalog.vue'
    import ThreadView from './ThreadView.vue'
</script>

<script>

    function dataRecalc() {
        return {
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
        this.dataRecalc()
        this.thrLoad()
    },
    watch: {
        '$route' (_to, _) {
            this.dataRecalc()
            this.thrLoad()
        }
    },
    methods: {
        ...mapActions({ getReq_Thread_ThrLoad: "getReq_Thread_ThrLoad", }),
        dataRecalc() {
            Object.assign(this.$data, dataRecalc())
        },
        thrLoad() {
            const N_LOAD = 30
            let board_url = this.boardUrl
            let op_post_n = this.thrN
            let from = this.next_n
            let n_load = N_LOAD
            this.getReq_Thread_ThrLoad({board_url, op_post_n, from, n_load}).then(res_x => {
                let res = res_x.loaded_posts
                let load_len = res.length                
                if (this.posts === null) {
                    this.posts = res
                } else {
                    this.posts = this.posts.concat(res)
                }
                
                this.next_n = this.next_n + load_len

                console.log('[thr load\'ed]', res, this.posts)
                console.log('[data]', res, this.$data)
            });
        },
    },
}
</script>

<template>
    <PageAwait v-if="posts === null && err == null" 
        :msgFull="'Выяс>>ня<<ем состояние ' + boardUrl + '#' + thrN" 
        :back-link="'/' + boardUrl + '/'"
        back-link-text="выяс>>ня<<ть"
    />
    <div class="board-inner" v-else-if="posts">
        <BoardHeader :boardName="'TODO'" :boardUrl="boardUrl" :isCatalog="true" />

        <ThreadView :posts="posts" :posts_qty="posts.length" :header="TODO" />
    </div>
    <PageNotFound v-else />
</template>

