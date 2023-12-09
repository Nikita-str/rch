<script setup>
import { msgUnpack, boardUrlCalc } from '../js/board_fns'
import { pad, rand_i } from '../js/fns'
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
    
    msgUnpacked() { return msgUnpack(boardUrlCalc(this), this.msg, this.nBoardOP) },
    noOpPic() {
        const ZEROS = 2
        const IMGS = ['jpg']
        let pic_n = rand_i(1, IMGS.length)
        return `/imgs/p_no_op/${pad(pic_n, ZEROS)}.${IMGS[pic_n - 1]}`
    },

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
    ctlgClassX() {
        return {
            'ctlg-post-top': this.isTop,
            'ctlg-post-std': !this.isTop,
        }
    },
    textClassX() { return 'ctlg-post-text-' + this.classPostfixX },
    msgClassX() { return 'ctlg-post-msg-' + this.classPostfixX },
  },
  methods: {
    imgClick() {
        let bUrl = boardUrlCalc(this)
        this.$router.push({ path: `/${bUrl}/${this.nBoardOP}/` })
    },

    OnMouseEnter() {
        if (this.isTop) return

        const rect = this.$refs.thePost.getBoundingClientRect();
        const top = rect.top + window.scrollY;
        const left = rect.left;

        let topInfo = {
            left,
            top,
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
            <PostPic v-if="imgInfo" :imgInfo="imgInfo" :noMarginRight="true" :picDimSz="picDimSzX" @img-click="imgClick" />
            <div v-else>
                <img class="ctlg-post-no-img pic-border" :src="noOpPic" alt="!no OP pic!" @click.left.prevent="imgClick">
                <p class="centered pic-spoiler-text" style="transform: translate(-50%, 150%) rotate(20deg);">!NO OP PIC!</p>
            </div>
        </div>
        <div class="ctlg-post-text" :class="[textClassX]">    
            <div class="ctlg-post-h" v-html="header" />
            <div class="ctlg-post-msg" :class="[msgClassX]" v-html="msgUnpacked" />
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

