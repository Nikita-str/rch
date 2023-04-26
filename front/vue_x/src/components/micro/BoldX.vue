
<script>
export default {
    // components: {
    //     BoldX
    // },
    props: ['val'],
    // props: {
    //     val: Object,
    // },
}
</script>

<template>
    <span v-if="val" style="font-weight:bold;">{{ val }}</span>
    <span v-else style="font-weight:bold;">???</span>
</template>