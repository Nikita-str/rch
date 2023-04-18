<script setup lang="ts">

defineProps<{
  msg: string,
  open_boards: number,
  posting_speed: number,
  total_post: number,
}>()

function Bnum(x: number) {
  return `<span style="font-weight:bold;">${x}</span>`
}
</script>

<script setup lang="ts">

</script>


<template>
  <p> {{ msg }} </p>
  <p style="margin-left: 5%;">Сейчас открыто аж <b>{{ open_boards }}</b> досок!</p>
  <!-- <p style="margin-left: 10%;">Скорость постинга всего-то <Bnum posting_speed>{{  }}</Bnum>, поднажмем!</p> -->
  <p style="margin-left: 10%;">Скорость постинга всего-то {{posting_speed}}, поднажмем!</p>
  <p style="margin-left: 15%;">За все время было <span style="font-weight:bold;">{{ total_post }}</span> постов!</p>
</template>
