<script setup>
import { lastUrlSubPath } from '@/js/board_fns'
import { computed } from 'vue'
import { useRouter } from 'vue-router';

const router = useRouter();

const last_url = computed(() => {
    console.log(router.currentRoute.value.path, lastUrlSubPath(router.currentRoute.value.path))
    return lastUrlSubPath(router.currentRoute.value.path)
})

const PATHS = ["full-load", "full-save", "shutdown"]
</script>


<template>
    <div style="margin-left: 0.6em;">
        <h4 class="ctrl-header" style="text-decoration: none;">вот ещё ctrl странички есть:</h4>
        <ul>
            <template v-for="path in PATHS">
                <li v-if="path != last_url">
                    <router-link :to="`../${path}/`" append>{{ path.replace('-', ' ') }}</router-link>
                </li>
            </template>
        </ul>
    </div>
</template>