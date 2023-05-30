<script setup>
    const SUBJ_MAX_LEN = 80;
    const MSG_PLACEHOLDER = 'Сообщи сообщение\nДоложи степень негодования';
</script>

<script> 
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
        async onSubmit(_x) {
            // TODO
        }
    },
}
</script>

<template>
    <form id="posting-from" ref="formSubmit" :action="formAction" method="post" >
        <div>
            <input type="hidden" name="board_url" :value="boardUrl" />
        </div>
        <div v-if="needSubj">
            <input type="text" name="post_header" tabindex="2" id="pf-subj" class="inp-x"  placeholder="тема" :maxlength="SUBJ_MAX_LEN" />
        </div>
        <div>
            <textarea          name="post_text"  tabindex="1" id="pf-msg" class="inp-x" :placeholder="MSG_PLACEHOLDER" rows="7" />
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

