<script setup>
    import PageNotFound from './PageNotFound.vue'
    import PageAwait from './PageAwait.vue'
    import BoardLoaded from './BoardLoaded.vue'
    import { trim } from '../js/fns'
    import { mapActions } from 'vuex'
</script>

<script>
    export default {
    data() {
        return {
            boardExist: null,
            boardUrl: trim(this.$route.path, "/"),
        };
    },
    mounted() {
        this.upd(this.$route.path);
    },
    methods: {
        ...mapActions({ getReqBoardName: "getReqBoardName", }),
        upd(new_path) {
            this.boardUrl = trim(new_path, "/");
            this.boardExist = this.$store.getters.getBoardName(this.boardUrl);
            if (this.boardExist === null) {
                this.getReqBoardName(this.boardUrl).then(res => {
                    this.boardExist = res;
                });
            }
        },
    },
    beforeRouteUpdate(to, from, next) {
        if (to.path != from.path) {
            this.upd(to.path);
            next();
        }
    },
}
</script>

<template>
    <PageAwait v-if="boardExist === null" :msg="'когда поймем что с /' + boardUrl + '/'" />
    <BoardLoaded v-else-if="boardExist.name" :boardName="boardExist.name" />
    <PageNotFound v-else />
</template>

