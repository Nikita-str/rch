
<!--template>
    <div class="posting-form-button">
        <slot />
    </div>
</template-->

<script>
export default {
    props: {
        onClick: {
            type: Function,
            default: null,
        },
    },
}
</script>

<template>
    <button class="posting-form-button" @click.left.stop.prevent="onClick">
        <slot />
    </button>
</template>

<style scoped>
    .posting-form-button {
        border: none;

        cursor: pointer;

        width: 1.6em;
        height: 1.6em;
        display: inline-block;
        
        text-align: center;
        vertical-align: middle;

        color: var(--r-col-bg-dark);
        background-color: var(--r-col-blue);
        margin: 3px;
        margin-left: 0px;
        margin-top: 0px;
        
        -webkit-clip-path: polygon(25% 0, 100% 0, 100% 75%, 75% 100%, 0 100%, 0 25%);
        clip-path: polygon(25% 0, 100% 0, 100% 75%, 75% 100%, 0 100%, 0 25%);
    }

    .posting-form-button:hover {
        color: var(--r-col-crab-light);
    }

</style>
