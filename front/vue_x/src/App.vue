<script>
export default {
  data(){
    return{
      _updCaled: false,
      openB: 42,
      total_post: this.$store.getters.getTotalPost,
    }
  },
  methods: {
    upd(){
      console.log("upd")
      this.$store.dispatch('updCommonInfo')
    },
    calcTotalPostFn() {
        console.log("upd by fn")
        this.upd()
        return this.$store.getters.getTotalPost
    },
  },
  computed: {
    calcTotalPost() {
        console.log("upd by computed")
        this.upd()
        return this.$store.getters.getTotalPost
    },
  },
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
        <About msg="Ура-ура общение!" :open_boards="openB" :posting_speed="0" :total_post="calcTotalPostFn()" />
        <router-link to="/tmp">X</router-link>
      </div>
    </div>
  </div>
</template>
