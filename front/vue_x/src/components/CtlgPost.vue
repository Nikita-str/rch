<script setup>
import { msgUnpack, boardUrlCalc } from '../js/board_fns'
import { pad, rand_i } from '../js/fns'
import PostPic from './files/pics/PostPic.vue'
import { defineEmits } from 'vue'
</script>

<script> 

export default {
  props: {
    header: {
        type: String,
        required: true,
    },
    msg: {
      type: String,
      required: true,
    },
    nBoardOP: {
      type: Number,
      required: true,
    },
    imgInfo: {
      // there can be only one img or none: first
      type: Object,
      default: null,
    },
  },
  computed: {
    msgUnpacked() { return msgUnpack(boardUrlCalc(this), this.msg, this.nBoardOP) },
    noOpPic() {
        const ZEROS = 2
        const IMGS = ['jpg']
        let pic_n = rand_i(1, IMGS.length)
        return `/imgs/p_no_op/${pad(pic_n, ZEROS)}.${IMGS[pic_n - 1]}`
    },
  },
  methods: {
    imgClick() {
        let bUrl = boardUrlCalc(this)
        this.$router.push({ path: `/${bUrl}/${this.nBoardOP}/` })
    },
  },
}
</script> 

<template>
    <div class="ctlg-post">
        <div class="ctlg-post-img">
            <PostPic v-if="imgInfo" :imgInfo="imgInfo" :noMarginRight="true" picDimSz="13em" @img-click="imgClick" />
            <div v-else>
                <img class="ctlg-post-no-img pic-border" :src="noOpPic" alt="!no OP pic!" @click.left.prevent="imgClick">
                <p class="centered pic-spoiler-text" style="transform: translate(-50%, 150%) rotate(20deg);">!NO OP PIC!</p>
            </div>
        </div>
        <div class="ctlg-post-text">    
            <div class="ctlg-post-h" v-html="header" />
            <div class="ctlg-post-msg" v-html="msgUnpacked" />
        </div>
        <!-- <div class="post-text" :style="{'padding-top': (imgInfo.length == 1) ? '2.4em' : '' }" v-html="msgUnpacked" /> -->
    </div>
</template>

<style>
.ctlg-post {
    position: relative;
    flex: 0 1 18em;
    cursor: pointer;
    background-color: var(--r-col-bg-dark);
    border: solid 1px var(--r-col-blue);
    margin: 0.3em;
    text-align: center;
}
.ctlg-post-img {
    display: inline-block;
    padding: 0.6em;
    height: 17em;
}
.ctlg-post-h {
    display: block;
    padding-left: 0.6em;
    padding-right: 0.6em;
    color: var(--r-col-blue);
    font-weight: 700;
}
.ctlg-post-msg {
    display: block;
    padding-left: 1em;
    padding-right: 1em;
    word-wrap: break-word;
    overflow: hidden;
    line-height: 1.2em;
    height: calc(7 * 1.2em);
}
.ctlg-post-text {
    height: calc(7 * 1.2em + 1.6em);
    margin-bottom: 0.3em;
}
.ctlg-post-no-img {
    max-width: 15.6em;
    max-height: 15.6em;
}
@media screen and (max-width: 1200px) {  /* TODO */ }
</style>

