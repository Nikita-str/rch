<script setup>
import { defineProps } from 'vue'
defineProps({ n: Number, })
</script>

<template>
    <span class="ctrl-line-N" :title="`pwd file: line ${n}`">L{{ n }}</span>
</template>

<style>
.ctrl-line-N {
    font-size: 0.65em;
    color: var(--a--nyan-color);
}
</style>