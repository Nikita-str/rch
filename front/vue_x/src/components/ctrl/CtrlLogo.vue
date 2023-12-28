<script setup>
import Logo from '@/components/Logo.vue'
const msg = "наконтролировались\nпора домой"
</script>

<template>
    <router-link to="/" class="b-head-logo">
        <Logo :msg="msg" :sz="0.8" />
    </router-link>
</template>