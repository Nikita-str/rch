<script setup>
import { img_ext_abbr } from '../../../js/pics/file_x'
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
            let init_name = this.imgInfo.name
            let name = (MAX_NAME_LEN < init_name.length) ? init_name.slice(0, MAX_NAME_LEN - 1) + '[..]' : init_name
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
    <figure :style="{'margin-right': single ? '2ch' : '6px'}">
        <figcaption class="post-pic-max-sz" style="overflow: hidden;" >
            <div style="height: 2.4em; display: flex; float: left; margin-right: 5px;">
                <div class="post-pic-info-strip" />
            </div>
            <div style="white-space: nowrap;">
                <a class="post-pic-caption-ref" :href="imgPath" :title="nameFull" target="_blank">{{ nameAbbr }}</a>
                <div class="post-pic-caption-info">{{ imgInfo.orig_kb_sz }} | {{ dimSz }}</div>
            </div>
        </figcaption>
        <a class="post-pic-a" :href="imgPath" target="_blank">
            <img class="post-pic-img post-pic-max-sz" :src="imgPathCompressed" :alt="dimSz">
        </a>
    </figure>
</template>

<style scoped>
.post-pic-max-sz {
    max-width: 15.4vw;
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
.post-pic-a:hover {
    background-color: #0000;
}
.post-pic-img {
    border: #0000 2px solid;
}
.post-pic-img:hover {
    /* border-color: var(--r-col-blue); */
    outline: solid 1px var(--r-col-blue);
}
</style>