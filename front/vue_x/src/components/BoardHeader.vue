<script setup>
import HorizontalLine from './micro/HorizontalLine.vue'
import Logo from './Logo.vue'
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
        offNewMsg: {
            type: String,
            default: null,
        },

    },
    // computed: { }
}
</script> 

<template>
    <div class="b-head">
        <div class="b-head-name" v-html="'/'+boardUrl+'/ : '+boardName" />
        <router-link v-if="isCatalog" class="b-head-catalog" :to="'/'+boardUrl+'/'">←←← на доску</router-link>
        <router-link v-else           class="b-head-catalog"   to="catalog/" append>→→→ каталог ←←←</router-link>
        
        <div class="b-head-new-thr">
            <span>[</span>
            <span v-if="!offNewMsg" @click="onNewThrClick"><router-link to="" append>{{headerNewMsg}}</router-link></span>
            <span v-else style="font-weight: bold;">{{offNewMsg}}</span>
            <span>]</span>
        </div>

        <router-link to="/" class="b-head-logo">
            <Logo msg="а чо там еще есть?" :sz="0.42" />
        </router-link>
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

.b-head-logo {
    position: absolute;
    right: 5vw;
    top: 0.3em;
}
.b-head-logo:hover { background-color: #0000; }


@media screen and (max-width: 600px) {
    .b-head-name, .b-head-catalog, .b-head-new-thr {
        padding-right: 30vw;
    }
    .b-head-logo {
        right: 1vw;
    }
}

.b-head-name {
    font-weight: 900;
}

</style>
