
<script>
export default {
    // components: {
    //     BoldX
    // },
    props: {
        val: {
            type: Object,
            default: () => {}
        }
  },
}
</script>

<template>
    <span style="font-weight:bold;">{{ val }}</span>
</template>