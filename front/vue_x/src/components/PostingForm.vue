<script setup>
    import { mapActions } from 'vuex'
    import {ref, toRaw } from "vue";

    import PostingFormButton from "./micro/PostingFormButton.vue";
    import DragAndDropField from "./files/pics/DragAndDropField.vue";
    import { make_reject_notify_txt } from "./files/pics/DragAndDropField.vue";
    import DragAndDropFieldX from "./files/DragAndDropField.vue";

    import { FileX } from "../js/pics/file_x";
    import { notific_ctor, NOTIFIC_TY_WARN } from "../js/elems/notific";
    import PicView from "./files/pics/View.vue";

    import { boardUrlCalc } from "../js/fns";

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
    /** if null => isNewThr */
    opPostN: {
        type: Number,
        required: true,
    },
    afterPostInThr: {
        type: Function,
        default: null,
    },
  },
  data() {
    return {
        tmpFiles: new Array(), //TODO: move into global
        msgMinWidthByFile: 2,
    }
  },
  computed: {
    // formAction() { return 'api/' + (this.isNewThr ? "board/thr_new" : "thread/post_new"); },
    needSubj() { return this.isNewThr },
    isNewThr() { return this.opPostN === null },
    boardUrl() { return boardUrlCalc(this.$route.path) },
  },
  methods: {
        ...mapActions({ postReq_Board_ThrNew: "postReq_Board_ThrNew", }),
        ...mapActions({ postReq_Thread_PostNew: "postReq_Thread_PostNew", }),
        onSubmit(x) {
            // this.$refs.formSubmit.submit()

            // console.log('TODO:DEL:raw[data][A]', postingForm.value.post_text)
            let el = document.getElementById(ID_POST_TEXT_FIELD);
            postingForm.value.post_text = el.value; // because `wrapSelected` not update v-modeled value
            let data = toRaw(postingForm.value);
            // console.log('TODO:DEL:raw[data][B]', data, postingForm.value.post_text)
            data.post_imgs = []
            for (const img of this.tmpFiles) {
                data.post_imgs.push(img.to_post_img())
            }
            
            console.log('TODO:DEL', data)

            this.tmpFiles = new Array()
            postingForm.value = newPostingFormContent();
            data.board_url = this.boardUrl;

            if (this.isNewThr) {
                this.postReq_Board_ThrNew(data).then(n => {
                    if (this.isNewThr && n !== null) {
                        this.$router.push('/' + this.boardUrl + "/" + n + '/')
                    } 
                });
            } else {
                data.op_post_n = this.opPostN
                this.postReq_Thread_PostNew(data).then(_ => {
                    if (this.afterPostInThr !== null) {
                        this.afterPostInThr()
                    }
                })
            }
            
            // x.target.reset()
        },
        recalcMsgWidth() {
            this.msgMinWidthByFile = (this.tmpFiles.length > 2) ? 3 : 2;
        },
        onSelected(files) {
            console.log('TODO:DEL:[pic-selected]:', files)
            
            new Promise(async (resolve) => {
                const MIN_DIM_SZ = 10
                const MAX_DIM_SZ = 250
                const canvas = document.createElement("canvas")
                let imgs = new Array()
                for (const file of files) {
                    let f = new FileX(file)
                    await f.add_base64()
                    await f.compress(canvas, MIN_DIM_SZ, MAX_DIM_SZ)
                    imgs.push(f)
                }
                canvas.remove()
                resolve(imgs)
            }).then((imgs) => {
                this.tmpFiles = this.tmpFiles.concat(imgs)
                this.recalcMsgWidth()
            })
            
        },
        onRejected(files) {
            console.log('TODO:DEL:[REJECT]:', files)
            for (const file of files) {
                notific_ctor(NOTIFIC_TY_WARN, make_reject_notify_txt(file), 4_000)
            }
        },
        onCancel(index) {
            console.log('TODO:DEL:[CANCEL]', index)
            this.tmpFiles.splice(index, 1)
            this.recalcMsgWidth()
        },
    },
}

export const ID_POST_TEXT_FIELD = "pf-msg";
function wrapSelected(before, after) {
    let el = document.getElementById(ID_POST_TEXT_FIELD)
    let start = el.selectionStart
    let end = el.selectionEnd
    let prefix = el.value.substring(0, start)
    let center = el.value.substring(start, end)
    let suffix = el.value.substring(end, el.value.length)

    // console.log('selection:', center);
    el.value = prefix + before + center + after + suffix
    el.selectionStart = start + before.length
    el.selectionEnd = end + before.length
    el.focus()
}
function wrapSelectedTag(tag) {
    wrapSelected(`[${tag}]`, `[/${tag}]`)
}
function addCmd(cmd) {
    var cmd = `[${cmd}]`
    let el = document.getElementById(ID_POST_TEXT_FIELD)
    let start = el.selectionStart
    let end = el.selectionEnd
    let prefix = el.value.substring(0, start)
    let suffix = el.value.substring(end, el.value.length)
    
    el.value = prefix + cmd + suffix
    // if (start != end) { } else { }
    el.selectionStart = start + cmd.length
    el.selectionEnd = start + cmd.length
    el.focus()
}
function diceStyle(dice) {
    let color = (dice == 20 || dice == 100) ? 'dx' : 'd'; 
    return `color: var(--rp--${color}-txt); background-color: var(--rp--d${dice}-bg); font-weight: bold;`
}
</script>

<template>
    <form id="posting-from" ref="formSubmit" v-on:submit.prevent="onSubmit">
        <div v-if="needSubj">
            <input
                type="text"
                v-model="postingForm.post_header"
                id="pf-subj"
                class="inp-x" 
                placeholder="тема"
                tabindex="1"
                :maxlength="SUBJ_MAX_LEN"
                autocomplete = "off"
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
                autocomplete = "off"
            />
        </div>
        <div>
            <PostingFormButton :onClick="() => wrapSelectedTag('b')"><b>B</b></PostingFormButton>
            <PostingFormButton :onClick="() => wrapSelectedTag('i')"><i>It</i></PostingFormButton>
            <PostingFormButton :onClick="() => wrapSelectedTag('s')"><s>St</s></PostingFormButton>
            <PostingFormButton :onClick="() => wrapSelectedTag('spoiler')"><span class="P-sp">?!</span></PostingFormButton>
            <PostingFormButton :onClick="() => wrapSelectedTag('sup')"><span style="margin-left: -2px;">A<sup>x</sup></span></PostingFormButton>
            <PostingFormButton :onClick="() => wrapSelectedTag('sub')"><span style="margin-left: -2px;">A<sub>x</sub></span></PostingFormButton>

            <input type="submit" id="pf-submit" value="Сделано!" class="inp-x" />
        </div>
        <div v-if="boardUrl == 'rp'">
            <PostingFormButton :onClick="() => addCmd('D4')"   :style="diceStyle(4)"  >4</PostingFormButton>
            <PostingFormButton :onClick="() => addCmd('D6')"   :style="diceStyle(6)"  >6</PostingFormButton>
            <PostingFormButton :onClick="() => addCmd('D8')"   :style="diceStyle(8)"  >8</PostingFormButton>
            <PostingFormButton :onClick="() => addCmd('D10')"  :style="diceStyle(10)" >10</PostingFormButton>
            <PostingFormButton :onClick="() => addCmd('D12')"  :style="diceStyle(12)" >12</PostingFormButton>
            <PostingFormButton :onClick="() => addCmd('D20')"  :style="diceStyle(20)" >20</PostingFormButton>
            <PostingFormButton :onClick="() => addCmd('D100')" :style="diceStyle(100)">X</PostingFormButton>
        </div>

        <DragAndDropField :needCompress=false @selected="onSelected" @rejected="onRejected" />
        <!-- <DragAndDropFieldX @selected="onSelected" /> -->

        <PicView v-if="tmpFiles.length > 0" :files="tmpFiles" @pic-cancel="(index) => { onCancel(index) }" />
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
    min-width: calc(max(calc(7 * 1.8em + 10ch + 7 * 3px - 12px), ( 3px + 6.4 * 15px ) * v-bind(msgMinWidthByFile)));
    max-width: calc(max(calc(7 * 1.8em + 10ch + 7 * 3px - 12px), 35vw));
    min-height: 6ch;
    max-height: calc(max(6ch, 70vh));
}
#pf-submit {
    height: 1.8em;
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

