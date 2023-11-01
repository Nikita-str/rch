
<script> 
export default {
  props: {
    margined: {
      type: Boolean,
      default: false,
    },
  },
  computed: {
    marginLeft() { return this.margined ? "0" : "5vw" },
  },
}
</script>


<template>
    <hr class="thr-view-horiz" />
</template>

<style scoped>
.thr-view-horiz {
    width: 90vw;
    margin-left: v-bind(marginLeft);
    margin-top: 0.6em;
    border-top: 1px solid var(--r-col-blue);
    border-bottom: 0px;
}
</style>