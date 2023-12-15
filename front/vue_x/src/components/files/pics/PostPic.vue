<script setup>
import { boardUrlCalc } from '@/js/board_fns'
import { img_ext_abbr } from '../../../js/pics/file_x'
import { spoilerPicPath } from '@/js/board_fns'
import { defineEmits } from 'vue'

defineEmits(['img-click'])
</script>

<script>
const MAX_NAME_LEN = 15
const IMG_PATH_PREFIX = "/imgs/pp"

export default {
    props: {
        imgInfo: {
            type: Object,
            required: true,
        },
        single: {
            type: Boolean,
            required: true,
        },
        noMarginRight: {
            type: Boolean,
            default: false,
        },
        noHoverBorder: {
            type: Boolean,
            default: false,
        },
        picDimSz: {
            type: String,
            default: null,
        },
        spoilerPicN: Number,
        imgPreviewRef: String,
    },
    computed: {
        spoiler() {
            return this.imgInfo.cf_ty == '#'
        },
        spoilerPic() { return spoilerPicPath(this.spoilerPicN) },
        bUrl() { return boardUrlCalc(this) },
        imgPathCompressed() {
            let ext = img_ext_abbr(this.imgInfo.cf_ty)
            return `${IMG_PATH_PREFIX}/${this.bUrl}/${this.imgInfo.n}_c.${ext}`
        },
        imgPathWoPrefix() {
            let ext = img_ext_abbr(this.imgInfo.f_ty)
            return `${this.imgInfo.n}.${ext}`
        },
        imgPath() {
            return `${IMG_PATH_PREFIX}/${this.bUrl}/${this.imgPathWoPrefix}`
        },
        imgRef() {
            if (this.imgPreviewRef) return this.imgPreviewRef
            return this.imgPath
        },
        nameAbbr() {
            let ext = img_ext_abbr(this.imgInfo.f_ty)
            let init_name = this.imgInfo.name
            let name = (MAX_NAME_LEN < init_name.length) ? init_name.slice(0, MAX_NAME_LEN - 1) + '[..]' : init_name
            return `${name}.${ext}`
        },
        nameFull() {
            let ext = img_ext_abbr(this.imgInfo.f_ty)
            return `${this.imgInfo.name}.${ext}`
        },
        /** longest case is `99 bytes` / `99.99 MB` */
        sz() {
            let sz = this.imgInfo.byte_sz
            if (!sz || sz == 0) {
                return `o_0?`
            } else if (sz < 100) {
                return `${sz} bytes`
            } else if (sz < 1024 * 100) {
                let kb = (sz / 1024).toFixed(2)
                return `${kb} KB`
            } else if (sz < 1024 * 512) {
                let kb = (sz / 1024).toFixed(0)
                return `${kb} KB`
            } else {
                let mb = (sz / 1024 / 1024).toFixed(2)
                return `${mb} MB`
            }
        },
        dimSz() {
            let w = this.imgInfo.w ? this.imgInfo.w : '???'
            let h = this.imgInfo.h ? this.imgInfo.h : '???'
            return `${w}x${h}`
        },
        picDimSzStyle() {
            if (this.picDimSz) {
                return {
                    maxWidth: this.picDimSz,
                    maxHeight: this.picDimSz,
                }
            } else {
                return null
            }
        },
        hoverBorder() {
            return this.noHoverBorder ? null : 'solid 1px var(--r-col-blue)';
        },
    },
    methods: {
        imgClick() {
            let info = {
                name: this.imgInfo.name,
                img_path: this.imgPathWoPrefix,
                expected_w: this.imgInfo.w ? this.imgInfo.w : 0,
                expected_h: this.imgInfo.h ? this.imgInfo.h : 0,
            }
            this.$emit('img-click', info)
        }
    },
}
</script>

<template>
    <figure :style="{'margin-right': noMarginRight ? null : (single ? '2ch' : '6px')}">
        <figcaption :class="{'post-pic-max-sz': picDimSz == null}" style="overflow: hidden;" >
            <div style="height: 2.4em; display: flex; float: left; margin-right: 5px;">
                <div class="post-pic-info-strip" />
            </div>
            <div style="white-space: nowrap;">
                <a class="post-pic-caption-ref" :href="imgPath" :title="nameFull" target="_blank">{{ nameAbbr }}</a>
                <div class="post-pic-caption-info">{{ sz }} | {{ dimSz }}</div>
            </div>
        </figcaption>
        <a class="post-pic-a" :href="imgRef" target="_blank" @click.left.prevent="imgClick">
            <div :class="{'post-pic-max-sz': picDimSz == null}" style="text-align: center;">
                <img class="post-pic-img pic-border" :style="picDimSzStyle" :src="spoiler ? spoilerPic : imgPathCompressed" :alt="dimSz">
                <p v-if="spoiler" class="centered pic-spoiler-text" style="transform: translate(-50%,-50%) rotate(-20deg);">!SPOILER!</p>
            </div>
        </a>
    </figure>
</template>

<style scoped>
.post-pic-max-sz {
    max-width: calc(min(15.4vw, 250px));
    min-width: 150px;
}
.post-pic-caption-ref {
    font-size: 1em;
    /* font-size: calc(min(1em, max(1.2vw, 0.6em))); */
    line-height: 1;
    white-space: nowrap;
    word-break: keep-all;
}
@media screen and (max-width: 1600px) and (min-width: 1200px) {
    .post-pic-caption-ref {
        font-size: 0.8em;
    }
}
@media screen and (max-width: 1200px) {
    .post-pic-caption-ref {
        font-size: 0.7em;
    }
    .post-pic-max-sz {
        max-width: 19vw;
    }
}
.post-pic-caption-info {
    font-size: 0.7em;
    color: var(--r-col-bg-light-blue);
    line-height: 1;
    margin-bottom: 3px;
}
.post-pic-info-strip {
    width: 2px;
    background: var(--r-col-crab-light);
    margin-left: 3px;
    
    margin-top: 6px;
    margin-bottom: 3px;
}
.post-pic-img {
    /* border: #0000 2px solid; */
    max-width: calc(min(max(15.4vw, 150px), 250px));
}
/* .post-pic-img:hover { outline: v-bind(hoverBorder); } */
</style>