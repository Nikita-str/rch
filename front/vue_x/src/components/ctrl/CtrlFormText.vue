<script setup>
import { defineEmits, ref } from 'vue'
defineEmits(['update:modelValue'])
const InputRef = ref(null)
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
        isNumber: {
            type: Boolean,
            default: false,
        },
    },
    computed: {
        inputValue: {
            get() { return this.modelValue; },
            set(newValue) { 
                this.$emit('update:modelValue', newValue)
                this.$refs.InputRef.value = newValue
            },
        }
    },
    methods: {
        inputChange(value) {
            if (this.isNumber) {
                this.inputValue = value.replace(/\D/g, '')
            } else {
                this.inputValue = value
            }
        }
    }
};
</script>

<template>
    <div>
        <label class="ctrl-label"><slot/></label>
        <span class="semitab"/>
        <input
            ref="InputRef"
            type="text"
            class="inp-x"
            :placeholder="placeholder"
            :tabindex="tab"
            :maxlength="maxLen"
            autocomplete = "off"

            :value="inputValue"
            @input="inputChange($event.target.value)"
        />
    </div>
</template>
