<script setup>
    import PageNotFound from './PageNotFound.vue'
    import PageAwait from './PageAwait.vue'
    import BoardLoaded from './BoardLoaded.vue'
    import { trim } from '../js/fns'
    import { mapActions } from 'vuex'
    import BoardHeader from './BoardHeader.vue'
    import Catalog from './Catalog.vue'
</script>

<script>
    export default {
    data() {
        return {
            boardExist: null,
            boardUrl: trim(this.$route.path, "/").split('/')[0],
        };
    },
    computed: {
        boardType() {
            const trim_path = trim(this.$route.path, "/").split('/')
            if (trim_path.length == 1) { return 'BoardLoaded' }
            else if (trim_path.length == 2) {
                if (trim_path[1] == `catalog`) { return 'Catalog' }
            } else { return `unkn` }
        },
    },
    mounted() {
        this.upd(this.$route.path);
    },
    methods: {
        ...mapActions({ getReq_Board_Name: "getReq_Board_Name", }),
        upd(new_path) {
            this.boardUrl = trim(new_path, "/").split('/')[0];
            this.boardExist = this.$store.getters.getBoardName(this.boardUrl);
            if (this.boardExist === null) {
                this.getReq_Board_Name(this.boardUrl).then(res => {
                    this.boardExist = res;
                });
            }
        },
    },
    watch: {
        '$route' (to, _) {
            this.upd(to.path)
        }
    },
}
</script>

<template>
    <PageAwait v-if="boardExist === null" :msg="'когда поймем что с /' + boardUrl + '/'" />
    <div class="board-inner" v-else-if="boardExist.name">
        <BoardHeader :boardName="boardExist.name" />

        <BoardLoaded      v-if="boardType == 'BoardLoaded'" :boardName="boardExist.name" />
        <Catalog     v-else-if="boardType == 'Catalog'"      />
        <div v-else class="board-sad-text">
            Inner error :3<br/>
            unknown board load type<br/>
            `{{boardType}}`
        </div>
    </div>
    <PageNotFound v-else />
</template>

