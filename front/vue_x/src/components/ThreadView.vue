
<script setup>
    import Post from './Post.vue'
</script> 

<script>
export default {
    props: {
        /** inner form: `[OP, last - n, last - (n - 1), ..., last - 1, last]` */
        posts: {
            type: Array,
            required: true,
        }
    },
}
</script> 

<template>
    <div class="thr-view">
        <div style="display: flex; padding-left: 1.2em;">
            <Post :msg="posts[0].text" :msgDate="posts[0].time" :msgBoardN="posts[0].n" :msgThrN="1" :msgWho="posts[0].poster" />
        </div>
        <hr class="thr-view-horiz" />
    </div>
</template>


<style scoped>
.thr-view {
    padding-left: 5%;
    margin-top: 1.2em;
}
.thr-view-horiz {
    width: 95%;
    margin-top: 0.5em;
    border-top: 1px solid var(--r-col-blue);
    border-bottom: 0px;
}
</style>
