<script setup>
import { pad, boardUrlCalc } from '../js/fns'
import PostPics from './files/pics/PostPics.vue'
import { defineEmits } from 'vue'

const emit = defineEmits(['post-n-click', 'img-click'])
function imgClick(info) { emit('img-click', info) }
</script>

<script> 

export default {
  props: {
    msg: {
      type: String,
      required: true,
    },
    msgDate: {
      type: Number | String,
      required: true,
    },
    msgBoardN: {
      type: Number,
      required: true,
    },
    msgThrN: {
      type: Number,
      required: true,
    },
    msgWho: {
      type: String,
      required: true,
    },
    msgReplies: {
      type: Array,
      required: false,
    },
    isOP: {
      type: Boolean,
      required: false,
      default: false,
    },
    nBoardOP: {
      type: Number,
      required: false,
      default: null,
    },
    imgsInfo: {
      type: Array,
      default: [],
    },
  },
  computed: {
    msgDateX() {
        if (typeof(this.msgDate) == 'string') { return this.msgDate }
        
        let date_time = new Date(this.msgDate * 1000);
        let hours = pad(date_time.getHours(), 2);
        let minutes = pad(date_time.getMinutes(), 2);
        let seconds = pad(date_time.getSeconds(), 2);
        let time = hours + ':' + minutes + ':' + seconds;

        let day = pad(date_time.getDate(), 2);
        let month = pad(date_time.getMonth(), 2);
        let year = pad(date_time.getFullYear(), 2);
        let date = day + '.' + month + '.' + year;

        return time + ' ' + date
    },
    xCssThrNumPaddingLeft() {
        let pad = (this.msgThrN > 99) ? 0 : ((this.msgThrN > 9) ? 1 : 2);
        return pad + "ch"
    },
    nBoardLink() {
      let bUrl = boardUrlCalc(this)
      if (this.isOP) {
        return '/' + bUrl + '/' + this.msgBoardN
      } else if (this.nBoardOP !== null) {
        return '/' + bUrl + '/' + this.nBoardOP + '#' + this.msgBoardN
      } else {
        return ""
      }
    },
    msgUnpacked() {
        var ret = ""
        var index = 0
        while(true) {
            const start = "<pkg "
            const i_start = this.msg.indexOf(start, index)
            if (i_start < 0) break

            const end = "></pkg>"
            const i_end = this.msg.indexOf(end, i_start + start.length)
            if (i_end < 0) break

            ret += this.msg.substring(index, i_start)
            let info = this.msg.substring(i_start + start.length, i_end).split(' ')
            let cmd = info[0]
            if (cmd == "reply") {
              let n = info[1]
              let maybe_OP = (n == this.nBoardOP) ? '(OP)' : ''
              
              let bUrl = boardUrlCalc(this)
              let href = '/' + bUrl + '/' + (this.isOP ? this.msgBoardN : this.nBoardOP)
              
              ret += `<a class="P-rep" href="${href}#${n}">&gt;&gt;${n}${maybe_OP}</a>`
            }
            
            index = i_end + end.length
        }
        ret += this.msg.substring(index)
        return ret
    },
    // msgThrNumX() {
    //     const PREFIX = '<span style="opacity:0;">' 
    //     const POSTFIX = '</span>' 
    //     let s = String(this.msgThrN);
    //     if (this.msgThrN > 99) {
    //         return s
    //     } else if (this.msgThrN > 9) {
    //         return PREFIX + '0' + POSTFIX + s
    //     } else {
    //         return PREFIX + '00' + POSTFIX + s
    //     }
    // }
  },
  methods: {
    onPostRefClick() { this.$emit('post-n-click', this.msgBoardN) }
  },
}
</script> 

<template>
    <div class="post" :class="{ 'post--not-op': !isOP }">
        <div class="post-header">
            <span class="post-who" v-html="msgWho"></span>
            <span class="post-date">{{ msgDateX }}</span>
            <span class="post-board-n">#<router-link :to="nBoardLink" @click="onPostRefClick">{{ msgBoardN }}</router-link></span>
            <span class="post-thr-n">{{ msgThrN }}</span>
        </div>
        <div class="post-inner">
              <PostPics :imgsInfo="imgsInfo" @img-click="imgClick" />
              <div class="post-text" :style="{'padding-top': (imgsInfo.length == 1) ? '2.4em' : '' }" v-html="msgUnpacked" />
        </div>
        <template v-if="msgReplies">
            <div class="post-replies">
                <template v-for="reply in msgReplies">
                    <a class="post-reply" href="#"> >>{{reply}}</a>
                </template>
            </div>
        </template>
    </div>
</template>

<style>
.post {
    /* max-width: 65vw; */
    max-width: 70vw;
    border-width: 1px;
    border-radius: 0;
    padding: 0.3em 1em 1.2em 0.6em;
}
@media screen and (max-width: 1200px) {
    .post {
        max-width: 85vw;
    }
}
.post--not-op {
    border-style: solid;
    /* background-color: var(--r-col-transparent-dbg); */
    background-color: var(--r-col-bg-dark);
    border-color: var(--r-col-blue);    
}

.post-header {
    white-space: nowrap;
}
.post-header > span + span {
    margin-left: 0.8em;
}
.post-thr-n {
    color: var(--r-col-blue);
    padding-left: v-bind(xCssThrNumPaddingLeft);
}

.post-inner {
    line-height: 1.4em;
    padding: 0em 0.2em 0em 1.5em;
}

.post-replies {
    padding-right: 0.6em;
    top: 0.5em;
    left: 1.5em;
    /* max-width: 90%; */
    overflow: auto;
    overflow-wrap: break-word;
}
.post-replies > a + a {
    margin-left: 0.8em;
}
.post-reply {
    font-size: 0.8em;
}

.post-text {
    word-wrap: break-word;
    display: inline-block;
}
</style>

