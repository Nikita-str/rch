<script setup>
import { defineEmits } from 'vue'
defineEmits(['update:modelValue'])
</script>

<script>
export default {
    props: {
        modelValue: Boolean,
        tab: Number,
    },
};
</script>

<template>
    <div>
        <label class="ctrl-label"><slot/></label>
        <span class="semitab"/>
        <input
            type="checkbox"
            class="x-checker nonselectable"
            :tabindex="tab"

            :checked="modelValue"
            @input="$emit('update:modelValue', $event.target.checked)"
        />
    </div>
</template>
