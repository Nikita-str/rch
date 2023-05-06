<script>
import { mapActions } from 'vuex'

export default {
    data() {
        return {
            popBoards: this.$store.getters.getPopBoards,
        }
    },
    mounted() {
        this.updPopBoards().then(() => {
            this.popBoards = this.$store.getters.getPopBoards
            console.log("after popBards")
        })
    },
    methods: { 
        ...mapActions({ updPopBoards: 'updPopBards', }),
    }
}
</script>

<template>
    <div v-if="popBoards" style="font-weight:bold;">
        <div v-for="{tag, boards} in popBoards">
            <ul>
                <li>{{ tag }}</li>
                <li v-for="{url, name} in boards">
                    <router-link :to="'/' + url + '/'">{{ name }}</router-link>
                </li>
            </ul>
            <br>
        </div>
    </div>
    <div v-else style="font-weight:bold;">...</div>
</template>

<style scoped>
ul {
    list-style: none;
}
</style>