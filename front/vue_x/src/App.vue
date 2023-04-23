<script>

export default {
  data(){
    return{
      openB: 42,
      total_post: this.$store.getters.getTotalPost,
    }
  }
}
</script>

<script setup>
import HelloWorld from './components/HelloWorld.vue'
import About from './components/About.vue'
</script>


<template>
  <div>
    <div style="height: 100%;">
      <div id="center">
        <HelloWorld msg="Незабывайте спать" />
        <About msg="Ура-ура общение!" :open_boards="openB" :posting_speed="0" :total_post="total_post" />
        <router-link to="/tmp">X</router-link>
      </div>
    </div>
  </div>
</template>
