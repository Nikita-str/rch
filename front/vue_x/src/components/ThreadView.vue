
<script setup>
import { msgUnpack, boardUrlCalc } from '@/js/board_fns'
import Post from './Post.vue'
import HorizontalLine from './micro/HorizontalLine.vue'
import AwaitDots from './micro/awaiters/BigAwaitDots.vue'
import { vElementVisibility } from '@vueuse/components'
import { defineEmits } from 'vue'

const emit = defineEmits(['post-n-click', 'img-click'])
function onPostRefClick(n) { emit('post-n-click', n) }
function imgClick(info) { emit('img-click', info) }
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
        bUrl() { return boardUrlCalc(this) },
    },
}
</script> 

<template><div>
    <div class="thr-view">
        <div>
            <div v-if="header" style="padding-left: 1.2em; color: var(--r-col-blue); font-weight: 700;" v-html="msgUnpack(bUrl, header, posts[0].n)" />
            <div style="display: flex; padding-left: 1.2em;">
                <Post 
                    :msg="posts[0].text"
                    :msgDate="posts[0].time"
                    :msgBoardN="posts[0].n"
                    :msgThrN="1"
                    :msgWho="posts[0].poster"
                    :msgReplies="posts[0].replies"
                    :isOP="true"
                    :imgsInfo="posts[0].imgs"

                    @post-n-click="onPostRefClick"
                    @img-click="imgClick"
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
                    :msgReplies="posts[post_index].replies"
                    :imgsInfo="posts[post_index].imgs"
                    :nBoardOP="posts[0].n"

                    @post-n-click="onPostRefClick"
                    @img-click="imgClick"
                />
                <Post v-else
                    :msg="posts[post_index].text" 
                    :msgDate="posts[post_index].time" 
                    :msgBoardN="posts[post_index].n" 
                    :msgThrN="msgThrNumHelper + post_index"
                    :msgWho="posts[post_index].poster"
                    :msgReplies="posts[post_index].replies"
                    :imgsInfo="posts[post_index].imgs"
                    :nBoardOP="posts[0].n"

                    v-element-visibility="(visible) => onNextLoadVis(visible, posts[post_index].n)"
                    @post-n-click="onPostRefClick"
                    @img-click="imgClick"
                />
                <!-- or default value for `onNextLoadVis` set to `(_) => {},` ? -->
                <!-- :msgThrN="999" for `msgThrN` padding test -->
            </div>
            <div id="thr-view-await-dots" v-if="!allLoaded">
                <AwaitDots />
            </div>
        </div>
    </div>
    <HorizontalLine />
</div></template>


<style scoped>
.thr-view {
    padding-left: 5%;
    margin-top: 1.2em;
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
