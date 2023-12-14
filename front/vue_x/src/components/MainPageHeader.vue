<script setup>
import { computed } from 'vue'
import Logo from './Logo.vue'

const BIG_MSG_DEFAULT = "Не забывайте спать<br/>"
+ "&nbsp;&nbsp;&nbsp;и быть счастливыми,<br/>"
+ "&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;также иногда надо кушать...<br/>"
+ "&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;...не забывайте.<br/>"
+ "Добра."
+ "<br/><br/>"
+ "Борда посвящена метахтонии, аниме, текстовому role play и поклонению слаяно-руничиским тентаклям."

const props = defineProps({
  miniMsg: {
    type: String,
    required: true
  },
  bigMsg: {
    type: String,
    default: null,
  }
})

const BigMsgX = computed(() => { return props.bigMsg ? props.bigMsg : BIG_MSG_DEFAULT })
</script>

<template>
  <div class="greetings">
    <div style="height: calc(1vh + 6px);" />
    <Logo :msg="miniMsg" />
    <div style="margin-left: 2ch; color: var(--r-col-bg-light-blue);" v-html="BigMsgX" />
    <img src="/imgs/board_related/preview.webp" style="width: 100%;" alt="board related preview pic" />
  </div>
</template>

