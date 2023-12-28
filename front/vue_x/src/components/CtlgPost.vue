<script setup>
import { msgUnpack, boardUrlCalc } from '../js/board_fns'
import { spoilerPicNum, noOpPicNum, noOpPicPath } from '@/js/board_fns'
import PostPic from './files/pics/PostPic.vue'
import { defineEmits, ref } from 'vue'

let thePost = ref(null)
const emit = defineEmits(['post-mouse-enter'])
</script>

<script> 

export default {
  props: {
    thrInfo : {
        type: Object,
        require: true,
    },
    topInfo : {
        type: Object,
        default: null,
    },
  },
  computed: {
    /** non-null */
    header() { return this.thrInfo.header },
    /** non-null */
    msg() { return this.thrInfo.msg },
    /** non-null */
    nBoardOP() { return this.thrInfo.nBoardOP },
    /** can be null if there no img \
     * there can be only one img or none: first
     */
    imgInfo() { return this.thrInfo.imgInfo },

    isTop() { return this.topInfo != null },
    classPostfixX() { return this.isTop ? 'top' : 'std' },

    spoilerPicN() { return this.topInfo ? this.topInfo.spoiler_n : spoilerPicNum() },
    noOpPicN() { return this.topInfo ? this.topInfo.no_op_n : noOpPicNum() },
    bUrl() { return boardUrlCalc(this) },
    imgPreviewRef() {
        let bUrl = this.bUrl
        return `/${bUrl}/${this.nBoardOP}/`
    },
    
    headerUnpacked() { return msgUnpack(this.bUrl, this.header, this.nBoardOP) },
    msgUnpacked() { return msgUnpack(this.bUrl, this.msg, this.nBoardOP) },
    noOpPic() { return noOpPicPath(this.noOpPicN) },

    picDimSzX() { return this.isTop ? "17em" : "13em" },
    divImgHeightX() { return this.isTop ? "20em" : "17em" },
    noImgSzX() { return this.isTop ? "19em" : "15.6em" },
    maybeTopStyle() {
        if (!this.topInfo) return null
        return {
            top: `calc(${this.topInfo.top}px - 1em)`,
            left: `calc(${this.topInfo.left}px - 2em)`,
        }
    },
    ctlgClassX() { return 'ctlg-post-' + this.classPostfixX },
    textClassX() { return 'ctlg-post-text-' + this.classPostfixX },
    msgClassX() { return 'ctlg-post-msg-' + this.classPostfixX },
  },
  methods: {
    imgClick() { this.$router.push({ path: this.imgPreviewRef }) },

    OnMouseEnter() {
        if (this.isTop) return

        const rect = this.$refs.thePost.getBoundingClientRect();
        const top = rect.top + window.scrollY;
        const left = rect.left;

        let topInfo = {
            left,
            top,
            spoiler_n: this.spoilerPicN,
            no_op_n: this.noOpPicN,
        }

        let thrInfo = this.thrInfo

        this.$emit('post-mouse-enter', thrInfo, topInfo)
    }
  },
}
</script> 

<template>
    <div ref="thePost" class="ctlg-post" :class="ctlgClassX" :style="maybeTopStyle" @mouseenter="OnMouseEnter">
        <div class="ctlg-post-img">
            <PostPic v-if="imgInfo" :imgInfo="imgInfo" :noMarginRight="true" :picDimSz="picDimSzX" @img-click="imgClick" :spoilerPicN="spoilerPicN" :imgPreviewRef="imgPreviewRef" />
            <div v-else>
                <a class="post-pic-a" :href="imgPreviewRef" target="_blank" @click.left.prevent="imgClick">
                    <img class="ctlg-post-no-img pic-border" :src="noOpPic" alt="!no OP pic!">
                    <p class="centered pic-spoiler-text" style="transform: translate(-50%, -175%) rotate(20deg);">!NO OP PIC!</p>
                </a>
            </div>
        </div>
        <div class="ctlg-post-text" :class="textClassX">    
            <div class="ctlg-post-h" v-html="headerUnpacked" />
            <div class="ctlg-post-msg" :class="msgClassX" v-html="msgUnpacked" />
        </div>
        <!-- <div class="post-text" :style="{'padding-top': (imgInfo.length == 1) ? '2.4em' : '' }" v-html="msgUnpacked" /> -->
    </div>
</template>

<style>
.ctlg-post {
    /* cursor: pointer; */
    background-color: var(--r-col-bg-dark);
    border: solid 1px var(--r-col-blue);
    text-align: center;
}
.ctlg-post-std {
    position: relative;
    flex: 0 1 18em;
    margin: 0.3em;
}
.ctlg-post-top {
    margin-top: 0.3em;
    position: absolute;
    width: 22em;
}

.ctlg-post-img {
    display: inline-block;
    padding: 0.6em;
    height: v-bind(divImgHeightX);
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
    line-height: 1.2em;
}
.ctlg-post-msg-std {
    overflow: hidden;
    height: calc(7 * 1.2em);
}
.ctlg-post-msg-top {
    min-height: calc(7 * 1.2em);
}

.ctlg-post-text {
    margin-bottom: 0.3em;
}
.ctlg-post-text-std {
    height: calc(7 * 1.2em + 1.6em);
}
.ctlg-post-text-top {
    min-height: calc(7 * 1.2em + 1.6em);
}

.ctlg-post-no-img {
    cursor: pointer;
    max-width: v-bind(noImgSzX);
    max-height: v-bind(noImgSzX);
}
@media screen and (max-width: 1200px) {  /* TODO */ }
</style>

