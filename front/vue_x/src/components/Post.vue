<script setup>
    import { pad } from '../js/fns'
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
    msgPic: {
      type: String,
      required: false,
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
  }
}
</script> 

<template>
    <div class="post" :class="{ 'post--not-op': !isOP }">
        <div class="post-header">
            <span class="post-who" v-html="msgWho"></span>
            <span class="post-date">{{ msgDateX }}</span>
            <span class="post-board-n">#<a href="#">{{ msgBoardN }}</a></span>
            <span class="post-thr-n">{{ msgThrN }}</span>
        </div>
        <div class="post-inner">
            <span v-if="msgPic" class="post-img">TODO</span>
            <span class="post-text" v-html="msg"></span>
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
    max-width: 70vw;
    border-width: 1px;
    border-radius: 0;
    padding: 0.3em 1em 1.2em 0.6em;
}
.post--not-op {
    border-style: solid;
    background-color: var(--r-col-transparent-dbg);
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
    padding: 0em 0.2em 0em 1.5em;
}

.post-replies {
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
}
</style>

