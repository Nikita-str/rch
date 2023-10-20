<script>
export default {
    props: {
        text: {
            type: String,
            required: true
        }
    }
}
</script>

<template>
    <div class="board-sad-text">{{ text }}</div>
</template>
