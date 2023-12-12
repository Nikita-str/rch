<script>
import { mapActions } from 'vuex'

export default {
  data(){
    return{
      totalPost: this.$store.getters.getTotalPost,
      openBoard: this.$store.getters.getOpenBoardAmount,
      speedPost: this.$store.getters.getSpeedPost,
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
        this.openBoard = this.$store.getters.getOpenBoardAmount
        this.speedPost = this.$store.getters.getSpeedPost
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
import MainPageHeader from './components/MainPageHeader.vue'
import About from './components/About.vue'
import PopBoards from './components/PopBoards.vue'
</script>

<template>
  <div style="height: 100%;">
    <div id="center">
      <MainPageHeader miniMsg="Незабывайте спать" />
      <About msg="Ура-ура общение!" :open_boards="openBoard" :posting_speed="speedPost" :total_post="totalPost" />
      <br/>
      <PopBoards />
      <router-link to="/tmp/">Invalid page :3</router-link>
    </div>
  </div>
</template>
