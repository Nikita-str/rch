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
    formAction() {
        return 'api/' + (this.isNewThr ? "board/thr_new" : "thread/post_new");
    },
    needSubj() { return this.isNewThr },
  },
  methods: {
        ...mapActions({ postReq_Board_ThrNew: "postReq_Board_ThrNew", }),
        onSubmit(x) {
            // this.$refs.formSubmit.submit()
            let data = toRaw(postingForm.value);
            postingForm.value = newPostingFormContent();
            data.board_url = this.boardUrl;
            this.postReq_Board_ThrNew(data).then(res => {
                console.log('[post req maded]', res)
            });
            // x.target.reset()
        }
    },
}
</script>

<template>
    <form id="posting-from" ref="formSubmit" :action="formAction" method="post" v-on:submit.prevent="onSubmit">
        <div>
            <input type="hidden" name="board_url" :value="boardUrl" />
        </div>
        <div v-if="needSubj">
            <input
                type="text"
                v-model="postingForm.post_header"
                name="post_header"
                id="pf-subj"
                class="inp-x" 
                placeholder="тема"
                tabindex="2"
                :maxlength="SUBJ_MAX_LEN"
            />
        </div>
        <div>
            <textarea
                v-model="postingForm.post_text"
                name="post_text" 
                id="pf-msg"
                class="inp-x"
                :placeholder="MSG_PLACEHOLDER"
                tabindex="1"
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
    padding: 3px;
}
#pf-subj {
    width: 42ch;
}
#pf-msg {
    width: 100%;
    min-width: 30ch;
    min-height: 5ch;
}
#pf-submit {
    float: right;
    color: var(--r-col-blue);
    border-radius: 0;
    font-weight: 900;
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

