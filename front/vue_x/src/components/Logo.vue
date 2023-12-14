<script setup>
import { defineProps, computed } from 'vue'
import { BOARD_NAME, BOARD_POSTFIX } from '@/js/settings'

const props = defineProps({
    msg: {
        type: String,
        default: null,
    },
    sz: {
        type: Number,
        default: 1.0,
    }
})

const scaleStyle = computed(() => { return { 'font-size': `${props.sz}em` } })
</script>

<template>
    <div id="logo" :style="scaleStyle">
        <div class="logo-board-fullname"><span class="logo-board-name">{{ BOARD_NAME }}</span>{{ BOARD_POSTFIX }}</div>
        <div v-if="msg" style="font-size: 1.3em;">{{ msg }}</div>
    </div>
</template>

<style scoped>
#logo {
    text-align: center;
}

.logo-board-fullname, .logo-board-name {
    font-weight: 900;
    top: -0.15em;
    margin-bottom: -0.56em;
}

.logo-board-fullname {
    color: var(--r-col-crab-light);
    font-size: 8em;
}
.logo-board-name {
    color: #000;
    font-size: 1em;
}
</style>