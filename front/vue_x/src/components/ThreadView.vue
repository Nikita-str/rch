
<script setup>
    import Post from './Post.vue'
    import AwaitDots from './micro/awaiters/BigAwaitDots.vue'
    import { vElementVisibility } from '@vueuse/components'
</script> 

<script>
const LAST_N_UPD = 7 // dunno between 5/10 so lets 7

export default {
    props: {
        /** inner form: `[OP, last - n, last - (n - 1), ..., last - 1, last]` */
        posts: {
            type: Array,
            required: true,
        },
        posts_qty: {
            type: Number,
            required: true,
        },
        header: {
            type: String,
            required: false,
        },
        allLoaded: {
            type: Boolean,
            default: true,
        },

        /** (visible: bool; post_board_n: number) */
        onNextLoadVis: {
            type: Function,
            default: null,
        }
    },
    computed: {
        msgThrNumHelper() {
            return this.posts_qty - (this.posts.length - 1)
        },
    },
}
</script> 

<template>
    <div class="thr-view">
        <div>
            <div v-if="header" style="padding-left: 1.2em; color: var(--r-col-blue); font-weight: 700;">{{ header }}</div>
            <div style="display: flex; padding-left: 1.2em;">
                <Post 
                    :msg="posts[0].text"
                    :msgDate="posts[0].time"
                    :msgBoardN="posts[0].n"
                    :msgThrN="1"
                    :msgWho="posts[0].poster"
                    :isOP="true"
                />
            </div>
            <div v-if="posts_qty > posts.length" class="thr-view-skip-info">пропущено постов: {{ posts_qty - posts.length }}</div>
            <div v-for="post_index in /*from 1 to*/ (posts.length - 1)" class="thr-view-reply">
                <Post v-if="allLoaded || onNextLoadVis === null || (post_index + LAST_N_UPD < posts.length)" 
                    :msg="posts[post_index].text" 
                    :msgDate="posts[post_index].time" 
                    :msgBoardN="posts[post_index].n" 
                    :msgThrN="msgThrNumHelper + post_index"
                    :msgWho="posts[post_index].poster"
                    :nBoardOP="posts[0].n"
                />
                <Post v-else
                    :msg="posts[post_index].text" 
                    :msgDate="posts[post_index].time" 
                    :msgBoardN="posts[post_index].n" 
                    :msgThrN="msgThrNumHelper + post_index"
                    :msgWho="posts[post_index].poster"
                    :nBoardOP="posts[0].n"

                    v-element-visibility="(visible) => onNextLoadVis(visible, posts[post_index].n)"
                />
                <!-- or default value for `onNextLoadVis` set to `(_) => {},` ? -->
                <!-- :msgThrN="999" for `msgThrN` padding test -->
            </div>
            <div id="thr-view-await-dots" v-if="!allLoaded">
                <AwaitDots />
            </div>
        </div>
        <hr class="thr-view-horiz" />
    </div>
</template>


<style scoped>
.thr-view {
    padding-left: 5%;
    margin-top: 1.2em;
}
.thr-view-horiz {
    width: 95%;
    margin-top: 0.6em;
    border-top: 1px solid var(--r-col-blue);
    border-bottom: 0px;
}

.thr-view-reply {
    display: flex;
    padding-left: 2.4em;
    margin-top: 0.3em;
}

.thr-view-skip-info {
    padding-left: 1.2em;
    color: var(--r-col-blue);
    opacity: 0.75;
}

#thr-view-await-dots {
    padding-left: 1.2em;
    margin-top: -1.2em;
    width: fit-content;
}
</style>
