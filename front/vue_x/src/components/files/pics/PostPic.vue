<script setup>
import { img_ext_abbr } from '../../../js/pics/file_x'
</script>

<script>
const MAX_NAME_LEN = 26
const IMG_PATH_PREFIX = "imgs/pp"

export default {
    props: {
        imgInfo: {
            type: Object,
            required: true,
        },
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
    <figure>
        <figcaption>
            <a :href="imgPath" :title="nameFull" target="_blank">{{ nameAbbr }}</a>
            <div>{{ imgInfo.orig_kb_sz }} | {{ dimSz }}</div>
        </figcaption>
        <a :href="imgPath" target="_blank">
            <img :src="imgPathCompressed" :alt="dimSz">
        </a>
    </figure>
</template>