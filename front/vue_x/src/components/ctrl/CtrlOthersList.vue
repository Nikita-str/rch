<script setup>
import { lastUrlSubPath } from '@/js/board_fns'
import { computed, defineProps } from 'vue'
import { useRouter } from 'vue-router';

const router = useRouter();

const last_url = computed(() => {
    return lastUrlSubPath(router.currentRoute.value.path)
})

const props = defineProps({
    isAll: {type: Boolean, required: false, default: false},
})

function to_url(path) {
    return props.isAll ? `./${path}/` : `../${path}/`
}

const PATHS = ["full-save", "full-load", "add-board", "del-post", "del-thr", "shutdown",]
</script>

<template>
    <div style="margin-left: 0.6em;">
        <h4 class="ctrl-header" style="text-decoration: none;">вот ещё ctrl странички есть:</h4>
        <ul>
            <template v-for="path in PATHS">
                <li v-if="isAll || path != last_url">
                    <router-link :to="to_url(path)" append>{{ path.replace('-', ' ') }}</router-link>
                </li>
            </template>
        </ul>
    </div>
</template>