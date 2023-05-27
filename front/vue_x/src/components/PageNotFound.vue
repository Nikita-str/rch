
<script setup>
// // this work but we need import all images and always load them ... bad decision
// // #tags: #static #load #img
// import pnf01 from '@/assets/img/page_not_found_01.png';

import '../assets/big_rect_page.css'

import { pad } from '../js/fns'; 
import { storage_circle_value } from '../js/fns_storage';

</script> 

<script> 
const PNF_ZEROS = 2; 

export default {
  data(){
    return{
      picN: () => {
        const ARR_KEY = "pnf_pseudo_rand";
        const INDEX_KEY = "pnf_pseudo_rand:index";
        const PNF_IMG = 3;
        return storage_circle_value(ARR_KEY, INDEX_KEY, PNF_IMG)
      },
    }
  },
}
</script> 

<template>
    <main>
        <router-view/>
        <div class="big-rect">
            <div class="big-rect-corn"></div>
            <div class="big-rect-inner" style="">
                <div class="big-rect-text">???</div>
                <img class="big-rect-img" :src="'/imgs/pnf/' + pad(picN(), PNF_ZEROS) + '.png'" />
                <div class="big-rect-text">Не-не-не, страница не найдена</div>
                <router-link class="big-rect-text" to="/">возвращаемся...</router-link>
            </div>
        </div>
    </main>
</template>

