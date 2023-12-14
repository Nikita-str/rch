<script setup>
    import PageNotFound from './PageNotFound.vue'
    import PageAwait from './PageAwait.vue'
    import BoardLoaded from './BoardLoaded.vue'
    import { trim } from '../js/fns'
    import { mapActions } from 'vuex'
    import BoardHeader from './BoardHeader.vue'
    import Catalog from './Catalog.vue'

    import DraggablePostingForm from '../components/DraggablePostingForm.vue'
    import BottomIndent from './micro/BottomIndent.vue'
</script>

<script>
    export default {
    data() {
        return {
            boardExist: null,
            boardUrl: trim(this.$route.path, "/").split('/')[0],
            draggableFormVisivle: false,
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
        isCatalog() {
            return this.boardType == 'Catalog'
        }
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
        onNewThrClick() {
            this.draggableFormVisivle = !this.draggableFormVisivle
        }
    },
    watch: {
        '$route' (to, _) {
            this.upd(to.path)
        }
    },
}
</script>

<template>
    <PageAwait v-if="boardExist === null" :msg="'Ждеееммм... когда поймем что с /' + boardUrl + '/'" />
    <div class="board-inner" v-else-if="boardExist.name">
        <BoardHeader :boardName="boardExist.name" :boardUrl="boardUrl" :isCatalog="isCatalog" :onNewThrClick="onNewThrClick" />

        <BoardLoaded      v-if="boardType == 'BoardLoaded'" :boardUrl="boardUrl"/>
        <Catalog     v-else-if="boardType == 'Catalog'"     :boardUrl="boardUrl"/>
        <div v-else class="board-sad-text">
            Inner error :3<br/>
            unknown board load type<br/>
            `{{boardType}}`
        </div>
        
        <BottomIndent />
    </div>
    <PageNotFound v-else />

    <DraggablePostingForm :boardUrl="boardUrl" :opPostN="null" :visible="draggableFormVisivle" />
</template>

