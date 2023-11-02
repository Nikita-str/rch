<script setup>
    import HorizontalLine from './micro/HorizontalLine.vue'
    import { boardUrlCalc } from '../js/fns'

    import { ref, onMounted, onUnmounted } from 'vue'

    // #####################################################

    const updText = ref('')
    let screenTypeListener

    function onScreenTypeChange(e) {
        updText.value = (e.matches ? '' : 'метохтоничное ') + 'обновление через'
    }

    onMounted(() => {
        screenTypeListener = window.matchMedia('(max-width: 800px)')
        onScreenTypeChange(screenTypeListener.matches)
        screenTypeListener.addEventListener('change', onScreenTypeChange)
    })

    onUnmounted(() => {
        screenTypeListener.removeEventListener('change', onScreenTypeChange)
    })
</script>

<script> 
const AUTO_UPD_DEFAULT = 30

// here in ref because should be shared between top and bottom bar
const autoUpd = ref(false)
const autoUpdIn = ref(AUTO_UPD_DEFAULT)

export default {
    // data() {
    //     return {
    //         autoUpdIn: AUTO_UPD_DEFAULT,
    //     }
    // },
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
        onCheckerChange() {
            if (autoUpd) {
                autoUpdIn.value = AUTO_UPD_DEFAULT >> 1
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
        <span class="thr-bar-elem thr-bar-rl">
            <input type="checkbox" class="thr-bar-auto-upd nonselectable" v-model="autoUpd" @change="onCheckerChange()" />
            <span>{{updText}}
                <span class="thr-bar-auto-upd-timer">{{ autoUpd ?  autoUpdIn : '...' }}</span>
            </span>
        </span>
        <!-- TODO: thread info (posts/pics/posters) -->
    </div>
    <HorizontalLine />
</template>

<style scoped>
.thr-bar {
    color: var(--r-col-blue);
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

.thr-bar-auto-upd {
    -webkit-appearance: none;
    appearance: none;
    margin: 0;
    margin-right: 0.5em;
    width: 1.2em;
    height: 1.2em;
    display: inline-block;
    vertical-align: sub;
    border: 3px solid var(--r-col-blue);
}
.thr-bar-auto-upd:hover {
    background-color: var(--r-col-crab-light);
}
.thr-bar-auto-upd:checked {
    background-color: var(--r-col-blue);
}
.thr-bar-auto-upd-timer {
    display: inline-block;
    text-align: right;
    min-width: 2.2ch;
}
</style>