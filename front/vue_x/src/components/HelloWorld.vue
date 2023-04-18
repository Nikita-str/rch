<script setup>
defineProps({
  msg: {
    type: String,
    required: true
  }
})
</script>

<template>
  <div class="greetings">
    <div style="height: calc(0.75vh + 4px);"></div>
    <div id="main-header-title">
      <h1 class="main-header"><font color="#000">R</font>ch</h1>
      <h3> {{ msg }} </h3>
    </div>
    <h4> {{ msg }} </h4>
    <span v-for="n in 50"><h3>{{ n }}</h3></span>
  </div>
</template>

<style scoped>
h1, h1>font {
  font-weight: 900;
  font-size: 7.4rem;
  top: -1rem;
  margin-bottom: -4rem;
  /* background-color: #0001; */
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

