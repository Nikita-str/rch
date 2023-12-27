<script setup>
import { defineEmits } from 'vue'
defineEmits(['update:modelValue'])
</script>

<script>
export default {
    props: {
        modelValue: String,
        tab: Number,
        placeholder: {
            type: String,
            default: "?",
        },
        maxLen: {
            type: Number,
            default: 80,
        },
    },
};
</script>

<template>
    <div>
        <label class="ctrl-label"><slot/></label>
        <span class="semitab"/>
        <input
            type="text"
            class="inp-x"
            :placeholder="placeholder"
            :tabindex="tab"
            :maxlength="maxLen"
            autocomplete = "off"

            :value="modelValue"
            @input="$emit('update:modelValue', $event.target.value)"
        />
    </div>
</template>
