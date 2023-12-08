<script setup>
import { msgUnpack, boardUrlCalc } from '../js/board_fns'
import { pad } from '../js/fns'
import PostPics from './files/pics/PostPics.vue'
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
    nBoardLink() {
        let bUrl = boardUrlCalc(this)
        if (this.isOP) {
            return '/' + bUrl + '/' + this.msgBoardN
        } else if (this.nBoardOP !== null) {
            return '/' + bUrl + '/' + this.nBoardOP + '#' + this.msgBoardN
        } else {
            return ""
        }
    },
    msgUnpacked() { return msgUnpack(boardUrlCalc(this), this.msg, this.nBoardOP) },
  },
  // methods: { },
}
</script> 

<template>
    <div class="ctlg-post">
        <div class="ctlg-post-img">
            <PostPic v-if="imgInfo" :imgInfo="imgInfo" :noMarginRight="true" />
        </div>
        <div class="ctlg-post-h" v-html="header" />
        <div class="ctlg-post-msg" v-html="msgUnpacked" />
        <!-- <div class="post-text" :style="{'padding-top': (imgInfo.length == 1) ? '2.4em' : '' }" v-html="msgUnpacked" /> -->
    </div>
</template>

<style>
.ctlg-post {
    position: relative;
    flex: 0 1 16em;
    cursor: pointer;
    background-color: var(--r-col-bg-dark);
    border: solid 1px var(--r-col-blue);
    margin: 0.3em;
    text-align: center;
    width: 20em;
}
.ctlg-post-img {
    display: inline-block;
    padding: 0.6em;
}
.ctlg-post-h {
    display: block;
    padding-left: 1em;
    padding-right: 1em;
    color: var(--r-col-blue);
    font-weight: 700;
}
.ctlg-post-msg {
    display: block;
    padding-left: 1.5em;
    padding-right: 1.5em;
    word-wrap: break-word;
    /* display: inline-block; */
    overflow: hidden;
    height: 8em;
}
@media screen and (max-width: 1200px) {  /* TODO */ }
</style>

