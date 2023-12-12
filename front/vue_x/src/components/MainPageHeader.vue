<script setup>
import { computed } from 'vue'

const BIG_MSG_DEFAULT = "Не забывайте спать<br/>"
+ "&nbsp;&nbsp;&nbsp;и быть счастливыми,<br/>"
+ "&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;также иногда надо кушать...<br/>"
+ "&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;...не забывайте.<br/>"
+ "Добра."
+ "<br/><br/>"
+ "Борда посвящена метахтонии, аниме, текстовому role play и поклонению слаянско-руничиским тентаклям."

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
    <div style="height: calc(0.75vh + 4px);"></div>
    <div id="main-header-title">
      <h1 class="main-header"><span style="color:#000">R</span>ch</h1>
      <h3> {{ miniMsg }} </h3>
    </div>
    <h4 style="margin-left: 2ch; color: var(--r-col-bg-light-blue);" v-html="BigMsgX" />
    <img src="/imgs/board_related/preview.webp" style="width: 100%;" alt="board related preview pic" />
  </div>
</template>

<style scoped>
h1, h1>span {
  font-weight: 900;
  font-size: 7.4rem;
  top: -1rem;
  margin-bottom: -4rem;
  /* background-color: #0001; */
}

h3, h4 {
  font-weight: normal;
}

h3 {
  font-size: 1.2rem;
  /* background-color: #0001; */
}

h4 {
  margin: 0px;
  padding: 0px;
  /* background-color: #0001; */
}

.greetings h1,
.greetings h3 {
  text-align: center;
}

/* #main-header-title {
  background-color: #9001;
} */

.greetings {
  line-height: 1.6;
}

/* 
@media (min-width: 1024px) {
  .greetings h1,
  .greetings h3 {
    text-align: left;
  }
} */
</style>

