<script>
import { mapActions } from 'vuex'

export default {
  data(){
    return{
      openB: 42,
      totalPost: this.$store.getters.getTotalPost,
    }
  },
  methods: {
    ...mapActions({
      updx: 'updCommonInfo'
    }),
    upd(){
      console.log("upd called")
      this.updx().then(() => {
        this.totalPost = this.$store.getters.getTotalPost
        console.log("after updx")
      })
    }
  },
  // computed: {
  //   calcTotalPost() {
  //       console.log("upd by computed")
  //       this.upd()
  //       return this.$store.getters.getTotalPost
  //   },
  // },
  mounted() {
    this.upd()
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
        <About msg="Ура-ура общение!" :open_boards="openB" :posting_speed="0" :total_post="totalPost" />
        <router-link to="/tmp">X</router-link>
      </div>
    </div>
  </div>
</template>
