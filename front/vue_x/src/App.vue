<script setup>
import HelloWorld from './components/HelloWorld.vue'
import TheWelcome from './components/TheWelcome.vue'
import About from './components/About.vue'
</script>

<template>
  <div>
    <div style="height: 100%;">
      <div id="center">
        <HelloWorld msg="Незабывайте спать" />
        <About msg="Ура-ура общение!" open_boards=42 posting_speed=0 total_post=0 />
      </div>
    </div>
  </div>
</template>
