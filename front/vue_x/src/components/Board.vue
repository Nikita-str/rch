<script setup>
import PageNotFound from '../components/PageNotFound.vue'
import PageAwait from '../components/PageAwait.vue'
import { mapActions } from 'vuex'
</script>

<script>
    import { trim } from '../js/fns';

    export default {
        data() {
            return {
                boardExist: null,
                boardUrl: trim(this.$route.path, '/'),
            }
        },
        mounted() {
            this.upd(this.$route.path)
        },
        methods: {
            ...mapActions({ getIsBoardExist: 'getIsBoardExist', }),
            upd(new_path) {
                this.boardUrl = trim(new_path, '/')
                this.boardExist = this.$store.getters.isBoardExist(this.boardUrl)
                if (this.boardExist === null) {
                    this.getIsBoardExist(this.boardUrl).then(res => {
                        this.boardExist = res
                    })
                }
            },
        },
        beforeRouteUpdate (to, from, next) {
            if (to.path != from.path) {
                this.upd(to.path)
                next()
            }
        },
    }
</script>

<template>
     <PageAwait v-if="boardExist === null" :msg="'когда поймем что с `' + boardUrl + '`'" />
    <div v-else-if="boardExist === true"> !!! </div>
    <PageNotFound v-else />
</template>

