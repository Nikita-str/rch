
<script setup>
  import BoardHeader from './BoardHeader.vue'
</script> 

<script> 
const PNF_ZEROS = 2; 

export default {
    props: {
        boardName: {
            type: String,
            required: true,
        }
    },
    data(){
        return{
        }
    },
}
</script> 

<template>
    <div class="board-inner">
        <BoardHeader :boardName="boardName" />
    </div>
</template>


<style scoped>
.board-inner {
    width: 100vw;
}
</style>
