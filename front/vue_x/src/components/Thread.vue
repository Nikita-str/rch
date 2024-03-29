<script setup>
    import PageNotFound from './PageNotFound.vue'
    import PageAwait from './PageAwait.vue'

    import AwaitDots from './micro/awaiters/BigAwaitDots.vue'
    import AwaitText from './micro/awaiters/BigAwaitText.vue'

    import { trim, isWhitespace } from '../js/fns'
    import { mapActions } from 'vuex'
    import BoardHeader from './BoardHeader.vue'
    import ThreadView from './ThreadView.vue'

    import { ID_POST_TEXT_FIELD } from '../components/PostingForm.vue'
    import DraggablePostingForm from '../components/DraggablePostingForm.vue'

    import BottomIndent from './micro/BottomIndent.vue'

    import ThreadBar from './ThreadBar.vue'

    import PicCloseView from './files/pics/PicCloseView.vue'
</script>

<script>

function dataRecalc() {
    return {
        boardExist: null,
        allLoaded: false,
        curLoad: false,
        header: null,
        posts: null,
        err: null,
        nextN: 0,

        draggableFormVisivle: false,

        picCloseViewInfo: new Object(),
        picCloseViewVisible: false,
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
        '$route' (to, from) {
            var from = trim(from.path.split('#')[0], '/')
            var to = trim(to.path.split('#')[0], '/')
            if (from != to) this.upd()
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
            const N_LOAD = 20 // 15 for test
            if (this.curLoad) { return }
            this.curLoad = true
            let board_url = this.boardUrl
            let op_post_n = this.thrN
            let from = this.nextN
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

                this.allLoaded = res_x.all_loaded
                this.nextN = this.nextN + load_len
                this.curLoad = false
            });
        },
        onNextLoadVisX(visible, post_board_n) {
            if (this.curLoad || !visible) {
                return
            }
            console.log('next load on post with baord number:', post_board_n)
            this.thrLoad()
        },
        onNewThrClick() {
            this.draggableFormVisivle = !this.draggableFormVisivle
        },
        onDpfClose() { this.draggableFormVisivle = false },
        onPostRefClickX(n) {
            this.draggableFormVisivle = true
            onPostRefClick(n)
        },
        onPostImgClick(info) {
            this.picCloseViewInfo = info
            this.picCloseViewVisible = true
        },
        onPicCloseViewClose() {
            this.picCloseViewVisible = false
        },
    },
}

function onPostRefClick(n) {
    let el = document.getElementById(ID_POST_TEXT_FIELD)
    let selection_start = el.selectionStart
    let selection_end = el.selectionEnd

    let text_start = el.value.substring(0, selection_start)
    let text_end = el.value.substring(selection_start)

    let need_space = (text_start.length == 0) ? false : !isWhitespace(text_start[text_start.length - 1])
    let prefix = (need_space ? ' ' : '') + ">>"
    let reply_text = `${prefix}${n}\n`


    el.value = text_start + reply_text + text_end
    el.selectionStart = selection_start + reply_text.length
    el.selectionEnd = selection_end + reply_text.length

    el.focus()
}
</script>

<template>
    <PageAwait v-if="boardExist === null" 
        :msgFull="'Выяс>>ня<<ем состояние ' + boardUrl + '#' + thrN" 
        back-link-text="выяс>>ня<<ть"
    />
    <div class="board-inner" v-else-if="boardExist.name">
        <BoardHeader 
            :boardName="boardExist.name" 
            :boardUrl="boardUrl" 
            :isCatalog="true" 
            :onNewThrClick="onNewThrClick" 
            headerNewMsg="Ответить в тред" 
            :offNewMsg="err ? 'Некуда отвечать' : (posts ? null : '???')"
        />
        <ThreadBar :upperBar="true" :onUpdate="thrLoad" :curLoad="curLoad" />

        <AwaitDots v-if="posts === null && err === null" />
        <AwaitText v-else-if="err && err.code == 1" :text="'доска /' + boardUrl + '/ не существует?!'" />
        <!-- TODO: доска удалена (получаеца) -->
        <AwaitText v-else-if="err && err.code == 2" :text="'тред не найден...'" />
        <!-- TODO: если посты были -- значит тред удален -->

        <ThreadView v-else 
            :posts="posts" 
            :posts_qty="posts.length" 
            :header="header" 
            :allLoaded="allLoaded"
            :onNextLoadVis="onNextLoadVisX"
            @post-n-click="onPostRefClickX"
            @img-click="onPostImgClick"
        />
        
        <ThreadBar :upperBar="false" :onUpdate="thrLoad" :curLoad="curLoad" />
        <BottomIndent />
    </div>
    <PageNotFound v-else />

    <DraggablePostingForm :boardUrl="boardUrl" :opPostN="parseInt(thrN)" :visible="draggableFormVisivle" :afterPostInThr="thrLoad" @close="onDpfClose" />
    <PicCloseView v-if="picCloseViewVisible" @close="onPicCloseViewClose" 
        :name="picCloseViewInfo.name"
        :img_path="picCloseViewInfo.img_path"
        :expected_w="picCloseViewInfo.expected_w"
        :expected_h="picCloseViewInfo.expected_h"
    />
</template>

