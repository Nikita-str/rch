<script setup>
    import HorizontalLine from './micro/HorizontalLine.vue'
    import { boardUrlCalc } from '../js/fns'
</script>

<script> 
export default {
  props: {
    upperBar: {
      type: Boolean,
      default: true,
    },
    onUpdate: {
        type: Function,
        default: null,
    },
  },
  computed: {
    boardUrl() { return boardUrlCalc(this.$route.path) },
    scrollText() { return this.upperBar ? "↓↓↓↓↓↓↓" : "↑↑↑↑↑↑↑" },
    scrollUrlPostfix() { return '#' + scrollPostfix(this.upperBar) }, // TODO: do smth with this
  },
    methods: {
        onScroll() {
            if (this.upperBar) {
                window.scrollTo(0, document.body.scrollHeight)
            } else {
                let el = document.getElementById(scrollId(!this.upperBar))
                el.scrollIntoView(true)
            }
        },
    },
}

function scrollPostfix(upperBar) {
    return upperBar ? "bottom" : "top"
}
function scrollId(upperBar) {
    return 'thread-bar-' + scrollPostfix(!upperBar)
}

</script>


<template>
    <div :id="scrollId(upperBar)" class="thr-bar">
        <span class="thr-bar-elem"><router-link class="thr-bar-rl" :to="'/'+boardUrl+'/'">←←←←←</router-link></span>
        <span class="thr-bar-elem" @click="onScroll"><router-link class="thr-bar-rl" :to="scrollUrlPostfix" append>{{ scrollText }}</router-link></span>
        <span class="thr-bar-elem" @click="onUpdate"><router-link class="thr-bar-rl" to="" append>обновить</router-link></span>
        <!-- TODO: auto update -->
        <!-- TODO: thread info (posts/pics/posters) -->
    </div>
    <HorizontalLine />
</template>

<style scoped>
.thr-bar {
    margin-top: 0.6em;
    padding-left: calc(5vw + 2px);
}
.thr-bar-elem {
    border-left: 2px solid var(--r-col-blue);
}
.thr-bar-elem:last-of-type {
    border-right: 2px solid var(--r-col-blue);
}
.thr-bar-rl {
    padding-left: 0.5em;
    padding-right: 0.5em;
}
</style>