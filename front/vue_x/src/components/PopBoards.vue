<script>
import { mapActions } from 'vuex'

const maxColumns = 4;

export default {
    data() {
        return {
            popBoards: this.popBoardsCalc(),
        }
    },
    mounted() {
        this.updPopBoards().then(() => {
            this.popBoards = this.popBoardsCalc()
            console.log("after popBards")
        })
    },
    methods: {
        ...mapActions({ updPopBoards: 'updPopBards', }),
        popBoardsCalc() {
            const tagged_boards = this.$store.getters.getPopBoards
            if (!tagged_boards) { return null }
            let columns = []
            let prev_h = 0;
            const tag_h = 2; /* tag + <br/> */
            let columns_h = []
            const c_index_last = maxColumns - 1;
            let c_index = c_index_last;
            for (const tag_board of tagged_boards) {
                if (columns.length == maxColumns) {
                    columns_h[c_index] += tag_board.boards.length + tag_h;
                    columns[c_index].push(tag_board);
                    let cur_h = columns_h[c_index];
                    if (cur_h >= prev_h) {
                        prev_h = cur_h;
                        c_index--;
                        if (c_index < 0) {
                            c_index = c_index_last;
                        }
                    }
                } else {
                    columns_h.push(tag_board.boards.length + tag_h)
                    columns.push([tag_board, ])
                }
            }

            return columns
        }
    }
}

</script>

<!-- <script setup>
import { ref, onMounted, onUnmounted } from 'vue'

const winW = ref(window.innerWidth)
const winH = ref(window.innerHeight)

const onResize = () => {
    winW.value = window.innerWidth
    winH.value = window.innerHeight
}

onMounted(() => { window.addEventListener('resize', onResize) })
onUnmounted(() => { window.removeEventListener('resize', onResize) })
</script> -->

<template>
    <p id="pop-board-header">Попупапипэлярные доски</p>

    <p v-if="popBoards && popBoards.length == 0" class="pop-baord-bad-msg">
        тут такое дело... вобщем их нет :((
    </p>
    <section v-else-if="popBoards" id="pop-board-section">
        <div class="pop-borad-column" v-for="pop_columns in popBoards">
            <template v-for="{tag, boards} in pop_columns">
                <ul>
                    <li class="pop-board-tag">{{ tag }}</li>
                    <li v-for="{url, name} in boards">
                        <router-link :to="'/' + url + '/'" v-html="name" />
                    </li>
                </ul>
                <br>
            </template>
        </div>
    </section>
    <div v-else class="pop-baord-bad-msg">...</div>
</template>

<style scoped>
ul {
    list-style: none;
    margin: 0;
}
li {
    text-align: left;
    white-space: nowrap;
}
#pop-board-header {
    text-align: center;
}    
#pop-board-section {
    display: flex;
    flex-flow: row wrap;
    padding-right: 24px;
}
.pop-borad-column {
    /* flex: 1 1 auto; */
    flex: 1;
}
.pop-board-tag {
    font-weight: 600;
}
.pop-baord-bad-msg {
    font-weight: bold;
    text-align: center;
}
</style>