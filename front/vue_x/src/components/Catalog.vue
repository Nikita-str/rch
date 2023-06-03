
<script setup>
    import { mapActions } from 'vuex'
    import { toRaw } from 'vue'
    import Post from './Post.vue'
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
    
    return {
        known_n,
        open_posts,
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
                console.log('[ctlg load\'ed]', res_x)
            });
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
    <div class="board-sad-text">tmp `{{ boardUrl }}`</div>
</template>


<style scoped>

</style>
