
<script setup>
    import { trim } from '../js/fns'
</script> 

<script>

function dataRecalc(new_path) {
    return {
        boardUrl: trim(new_path, "/").split('/')[0],
    }
}

export default {
    props: {
        boardName: {
            type: String,
            required: true,
        }
    },
    data(){ return dataRecalc(this.$route.path) },
    methods: {
        dataRecalc(new_path) {
            Object.assign(this.$data, dataRecalc(new_path))
        },
    },
    watch: {
        '$route' (to, _) {
            this.dataRecalc(to.path)
        }
    },
}
</script> 

<template>
    <div class="b-head">
        <div class="b-head-name">/{{boardUrl}}/ : {{ boardName }}</div>
        <router-link class="b-head-catalog" to="catalog/" append>→→→ каталог ←←←</router-link>
        <div class="b-head-new-thr">[<router-link to="" append>Создать тред</router-link>]</div>
        <hr class="b-head-horiz" />
    </div>
</template>


<style scoped>
.b-head-horiz {
    width: 90%;
    margin: auto;
    margin-top: 0.7em;
    border-top: 1px solid var(--r-col-blue);
    border-bottom: 0px;
}

.b-head {
    width: 100%;
    text-align: center;
    font-size: 1.45em;
    line-height: 1.3;
    padding-top: 0.7em;
    color: var(--r-col-blue);
}

.b-head-name {
    font-weight: 900;
}
</style>
