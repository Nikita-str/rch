<script setup>
    import { mapActions } from 'vuex'
    import {ref, toRaw } from "vue";

    const SUBJ_MAX_LEN = 80;
    const MSG_PLACEHOLDER = 'Сообщи сообщение\nДоложи степень негодования';
</script>

<script> 
function newPostingFormContent() {
    return {
        post_header: null,
        post_text: null,
    }
}
const postingForm = ref(newPostingFormContent())

export default {
  props: {
    boardUrl: {
      type: String,
      required: true,
    },
    isNewThr: {
      type: Boolean,
      required: true,
    },
  },
  computed: {
    // formAction() { return 'api/' + (this.isNewThr ? "board/thr_new" : "thread/post_new"); },
    needSubj() { return this.isNewThr },
  },
  methods: {
        ...mapActions({ postReq_Board_ThrNew: "postReq_Board_ThrNew", }),
        ...mapActions({ postReq_Thread_PostNew: "postReq_Thread_PostNew", }),
        onSubmit(x) {
            // this.$refs.formSubmit.submit()
            let data = toRaw(postingForm.value);
            postingForm.value = newPostingFormContent();
            data.board_url = this.boardUrl;
            if (this.isNewThr) {
                this.postReq_Board_ThrNew(data).then(n => {
                    if (this.isNewThr && n !== null) {
                        this.$router.push('/' + this.boardUrl + "/" + n + '/')
                    } 
                });
            } else {
                this.postReq_Thread_PostNew(data)
            }
            
            // x.target.reset()
        }
    },
}
</script>

<template>
    <form id="posting-from" ref="formSubmit" v-on:submit.prevent="onSubmit">
        <div>
            <input type="hidden" name="board_url" :value="boardUrl" />
        </div>
        <div v-if="needSubj">
            <input
                type="text"
                v-model="postingForm.post_header"
                id="pf-subj"
                class="inp-x" 
                placeholder="тема"
                tabindex="1"
                :maxlength="SUBJ_MAX_LEN"
            />
        </div>
        <div>
            <textarea
                v-model="postingForm.post_text"
                id="pf-msg"
                class="inp-x"
                :placeholder="MSG_PLACEHOLDER"
                tabindex="2"
                rows="7"
            />
        </div>
        <div>
            <input type="submit" id="pf-submit" value="Сделано!" class="inp-x" />
        </div>
    </form>
</template>

<style>
#posting-from {
    padding: 1px 3px 0px 3px;
}
#pf-subj {
    /* width: 42ch; */
    width: 100%;
}
#pf-msg {
    width: 42ch;
    min-width: 30ch;
    max-width: calc(max(30ch, 30vw));
    min-height: 5ch;
    max-height: calc(max(5ch, 70vh));
}
#pf-submit {
    float: right;
    color: var(--r-col-blue);
    border-radius: 0;
    font-weight: 900;
    margin-bottom: 3px;
}
#pf-submit:hover {
  background: var(--r-col-crab-light);
}
.inp-x {
    border: 1px solid var(--r-col-blue-70);
}
.inp-x::placeholder {
    color: var(--r-col-blue);
}
</style>

