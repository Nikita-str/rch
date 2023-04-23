<script>

export default {
  data(){
    return{
      openB: 42,
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
        <About :open_boards="openB" :posting_speed="0" :total_post="0" msg="Ура-ура общение!" />
      </div>
    </div>
  </div>
</template>
