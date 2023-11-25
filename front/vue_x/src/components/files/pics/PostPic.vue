<script setup>
import { img_ext_abbr } from '../../../js/pics/file_x'
</script>

<script>
const MAX_NAME_LEN = 26
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
        }
    },
    computed: {
        imgPathCompressed() {
            let ext = img_ext_abbr(this.imgInfo.cf_ty)
            return `${IMG_PATH_PREFIX}/${this.imgInfo.n}_c.${ext}`
        },
        imgPath() {
            let ext = img_ext_abbr(this.imgInfo.f_ty)
            return `${IMG_PATH_PREFIX}/${this.imgInfo.n}.${ext}`
        },
        nameAbbr() {
            let ext = img_ext_abbr(this.imgInfo.f_ty)
            let name = this.imgInfo.name.slice(0, MAX_NAME_LEN)
            return `${name}.${ext}`
        },
        nameFull() {
            let ext = img_ext_abbr(this.imgInfo.f_ty)
            return `${this.imgInfo.name}.${ext}`
        },
        dimSz() {
            return `${this.imgInfo.orig_w}x${this.imgInfo.orig_h}`
        },
    }
}
</script>

<template>
    <figure :style="{'margin-right': single ? '2ch' : '10px'}">
        <div style="height: 2.4em; display: flex; float: left; margin-right: 5px;">
            <div class="post-pic-info-strip" />
        </div>
        <figcaption>
            <a class="post-pic-caption-ref" :href="imgPath" :title="nameFull" target="_blank">{{ nameAbbr }}</a>
            <div class="post-pic-caption-info">{{ imgInfo.orig_kb_sz }} | {{ dimSz }}</div>
        </figcaption>
        <a :href="imgPath" target="_blank">
            <img :src="imgPathCompressed" :alt="dimSz">
        </a>
    </figure>
</template>

<style scoped>
.post-pic-caption-ref {
    font-size: 1em;
    line-height: 1;
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
</style>