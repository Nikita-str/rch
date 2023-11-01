<script setup>
    import HorizontalLine from './micro/HorizontalLine.vue'
</script>

<script>
export default {
    props: {
        boardName: {
            type: String,
            required: true,
        },
        boardUrl: {
            type: String,
            required: true,
        },
        isCatalog: {
            type: Boolean,
            required: true,
        },
        onNewThrClick: {
            type: Function,
            default: null,
        },
        headerNewMsg: {
            type: String,
            default: "Создать тред",
        },
    },
    computed: {
        catalogRouteTo() {
            if (this.isCatalog) { return '' }
            else { return 'catalog/' }
        },
    }
}
</script> 

<template>
    <div class="b-head">
        <div class="b-head-name" v-html="'/'+boardUrl+'/ : '+boardName" />
        <!-- <router-link class="b-head-catalog" :to="catalogRouteTo" append>→→→ каталог ←←←</router-link> -->
        <router-link v-if="isCatalog" class="b-head-catalog" :to="'/'+boardUrl+'/'">←←← на доску</router-link>
        <router-link v-else           class="b-head-catalog"   to="catalog/" append>→→→ каталог ←←←</router-link>
        
        <div class="b-head-new-thr">[<span @click="onNewThrClick"><router-link to="" append>{{headerNewMsg}}</router-link></span>]</div>
        <HorizontalLine />
    </div>
</template>


<style scoped>
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
