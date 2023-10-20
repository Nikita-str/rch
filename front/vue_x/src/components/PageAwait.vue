
<script setup>
import '../assets/big_rect_page.css'

import { pad } from '../js/fns'; 
import { storage_circle_value } from '../js/fns_storage';

</script> 

<script> 
const PNF_ZEROS = 2; 

export default {
  props: {
    msgFull: {
      type: String,
      required: false,
    },
    msg: {
      type: String,
      required: false,
      default: "Ждеееммм...",
    },
    backLinkText: {
      type: String,
      required: false,
      default: "ждать",
    },
    backLink: {
      type: String,
      required: false,
      default: '/',
    },
  },
  data(){
    return{
      picN: () => {
        const ARR_KEY = "pawait_pseudo_rand";
        const INDEX_KEY = "pawait_pseudo_rand:index";
        const PNF_IMG = 2;
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
            <div class="big-rect-inner">
              <div class="big-rect-text">
                <template v-if="msgFull">{{ msgFull }}</template>
                <template v-else>{{msg}}</template>
              </div>
              <img class="big-rect-img" :src="'/imgs/pawait/' + pad(picN(), PNF_ZEROS) + '.png'" />
              <router-link class="big-rect-text" :to='backLink' style="top: 1em;">устали {{backLinkText}}!</router-link>
            </div>
        </div>
    </main>
</template>

