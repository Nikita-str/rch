
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
        <div>
            <div style="display: flex; padding-left: 1.2em;">
                <Post :msg="posts[0].text" :msgDate="posts[0].time" :msgBoardN="posts[0].n" :msgThrN="1" :msgWho="posts[0].poster" />
            </div>
            <div v-for="post_index in /*from 1 to*/ (posts.length - 1)" class="thr-view-reply">
                <Post 
                    :msg="posts[post_index].text" 
                    :msgDate="posts[post_index].time" 
                    :msgBoardN="posts[post_index].n" 
                    :msgThrN="999" 
                    :msgWho="posts[post_index].poster"
                />
            </div>
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
    margin-top: 0.6em;
    border-top: 1px solid var(--r-col-blue);
    border-bottom: 0px;
}

.thr-view-reply {
    display: flex;
    padding-left: 2.4em;
    margin-top: 0.3em;
}
</style>
