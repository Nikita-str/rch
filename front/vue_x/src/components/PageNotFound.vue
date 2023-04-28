
<script setup>
// // this work but we need import all images and always load them ... bad decision
// // #tags: #static #load #img
// import pnf01 from '@/assets/img/page_not_found_01.png';

import { i_rand, pad } from '../js/fns'; 
</script> 

<template>
    <main>
        <router-view/>
        <div id="page-not-found">
            <div class="corn"></div>
            <div class="pnf-inner" style="">
                <div class="pnf-text">???</div>
                <img class="pnf-img" :src="'/imgs/pnf/' + pad(i_rand(1, 3), 2) + '.png'" />
                <div class="pnf-text">Не-не-не, страница не найдена</div>
                <router-link class="pnf-text" to="/">возвращаемся...</router-link>
            </div>
        </div>
    </main>
</template>

<style>
#page-not-found {
    background: var(--r-col-transparent-dbg);
    /* background: transparent; */
    display: flex;
    overflow: hidden;
    width: calc(max(480px, 40vw));
    height: calc(max(480px, 40vh));
    margin-left: calc((100vw - max(480px, 40vw)) / 2);
    margin-top: calc((100vh - max(480px, 40vh)) / 2);
}
/* #page-not-found>.bg {
    width: 200%;
    height: 200%;
    background: var(--r-col-transparent-dbg);
    position: absolute;
    z-index: -1;
    margin-left: -50%;
    margin-top: -50%;
    transform-origin: 50% 50%;
    transform: rotate(45deg) translateX(-17%);
} */
#page-not-found>.corn {
    background: var(--r-col-bg);
    position: absolute;
    z-index: 1;
    /* width: 50px;
    height: 50px;
    right: -30px;
    bottom: -30px; */
    width: calc(max(50px,  ( 5vw + 5vh ) / 2));
    height: calc(max(50px,  ( 5vw + 5vh ) / 2));
    right: calc(min(-30px, ( -3.0vw + -3.0vh ) / 2));
    bottom: calc(min(-30px, ( -3.0vw + -3.0vh ) / 2));
    transform-origin: 50% 50%;
    transform: rotate(45deg);
}
.pnf-inner {
    width: 100%; 
    height: 100%; 
    display: block;
    text-align: center;
    justify-content: center;
}
.pnf-img {
    display: block;
    margin-left: auto;
    margin-right: auto;
    height: 80%;
}
.pnf-text {
    height: 6.5%;
    font-weight: bold;
    font-size-adjust: ex-height;
}
</style>

