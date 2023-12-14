
<script setup>
    import { mapActions } from 'vuex'
    import { toRaw } from 'vue'
    import CtlgPost from './CtlgPost.vue'
</script> 

<script>
const SEST_KEY_PREFIX = "Catalog.vue";

function dataRecalc(new_boardUrl) {
    const sest_board = ':' + new_boardUrl;

    let open_posts
    let open_posts_json = sessionStorage.getItem(SEST_KEY_PREFIX + sest_board + ':open_posts')
    if (open_posts_json) {
        open_posts = JSON.parse(open_posts_json);
    } else {
        open_posts = new Array()
    }

    let known_n = new Set(open_posts.map(item => item.open_post.n));
    console.log('[TODO:DEL]', known_n, open_posts)
    
    // TODO: diff arrays of sort for open_posts!!
    // bump_order: Array<Index to `open_posts`>
    // time_order: Array<Index to `open_posts`>
    // and so on !!

    return {
        known_n,
        open_posts,
        topCtlgPost: null,
    }
}

export default {
    props: {
        boardUrl: {
            type: String,
            required: true,
        },
    },
    methods: {
        dataRecalc(new_boardUrl) {
            Object.assign(this.$data, dataRecalc(new_boardUrl))
        },

        ...mapActions({ postReq_Board_CtlgLoad: "postReq_Board_CtlgLoad", }),
        thrLoad() {
            let board_url = this.boardUrl;
            let known_n = Array.from(toRaw(this.known_n));
            
            this.postReq_Board_CtlgLoad({board_url, known_n}).then(res_x => {
                console.log('TODO:DEL:[ctlg load\'ed]', res_x)

                for (const thr of res_x.new_thrs) {
                    this.open_posts.push({
                        header: thr.header,
                        msg: thr.open_post.text,
                        nBoardOP: thr.open_post.n,
                        imgInfo: thr.open_post.imgs[0],
                    })
                }
            });
        },

        removeTopCtlgPost() { this.topCtlgPost = null },
        addTopCtlgPost(thrInfo, topInfo) {
            this.topCtlgPost = {
                thrInfo,
                topInfo,
            }
        },
    },
    data() { return dataRecalc(this.boardUrl) },
    mounted() { this.thrLoad() },
    watch: {
        boardUrl(new_boardUrl, _) {
            this.dataRecalc(new_boardUrl)
            this.thrLoad()
        },
    },
}
</script> 

<template>
    <div style="height: 0.3em;" />
    <div class="ctlg-posts">
        <div class="ctlg-posts-inner">
            <template v-for="thr in open_posts">
                <CtlgPost :thrInfo="thr" @post-mouse-enter="addTopCtlgPost" />
            </template>
        </div>
    </div>
    <CtlgPost v-if="topCtlgPost" :thrInfo="topCtlgPost.thrInfo" :topInfo="topCtlgPost.topInfo" @mouseleave="removeTopCtlgPost" />
</template>


<style>
.ctlg-posts {
    width: 100vw;
    display: inline-flex;
    /* flex-flow: wrap; */
    flex-flow: column;
    padding-left: 5vw;
    padding-right: 5vw;
    justify-content: center;
}
.ctlg-posts-inner {
    display: inline-flex;
    flex-flow: wrap;
    justify-content: center;
}
</style>
