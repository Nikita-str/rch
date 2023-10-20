
<template>
    <div class="board-sad-text">
        <span class="board-await" style="animation-delay: 0s;">.</span>
        <span class="board-await" style="animation-delay: 1s;">.</span>
        <span class="board-await" style="animation-delay: 2s;">.</span>
    </div>
</template>


<style scoped>
@keyframes animx-await-change-color {
  from { color: var(--r-col-blue); }
  20%   { color: var(--r-col-transparent-dbg); }
  40%  { color: var(--r-col-transparent-dbg); }
  60%  { color: var(--r-col-blue); }
  to   { }
}
.board-await {
    animation-name: animx-await-change-color;
    animation-duration: 4s;
    animation-iteration-count: infinite;
    animation-direction: normal;
}
</style>