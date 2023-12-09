import { trim, pad, rand_i } from './fns'

/** 
 * @param obj object with $route
 * @return boardUrl
*/
export function boardUrlCalc(obj) {
    var path = (typeof(obj) == 'string') ? obj : obj.$route.path;
    return trim(path, "/").split('/')[0]
}

/** 
 * @param {String} bUrl url of board (just call `boardUrlCalc` for it)
 * @param {String} msg
 * @param {Number} nBoardOP OP board n
 * @return {String} unpacked msg
*/
export function msgUnpack(bUrl, msg, nBoardOP) {
    var ret = ""
        var index = 0
        while(true) {
            const start = "<pkg "
            const i_start = msg.indexOf(start, index)
            if (i_start < 0) break

            const end = "></pkg>"
            const i_end = msg.indexOf(end, i_start + start.length)
            if (i_end < 0) break

            ret += msg.substring(index, i_start)
            let info = msg.substring(i_start + start.length, i_end).split(' ')
            let cmd = info[0]
            if (cmd == "reply") {
              let n = info[1]
              let maybe_OP = (n == nBoardOP) ? '(OP)' : ''
              
              let href = '/' + bUrl + '/' + nBoardOP
              
              ret += `<a class="P-rep" href="${href}#${n}">&gt;&gt;${n}${maybe_OP}</a>`
            }
            
            index = i_end + end.length
        }
        ret += msg.substring(index)
        return ret
}


const SPOILER_ZEROS = 2
const SPOILER_IMGS = ['webp', 'jpg']
export function spoilerPicNum() {
    let pic_n = rand_i(1, SPOILER_IMGS.length)
    return pic_n
}
export function spoilerPicPath(pic_n = null) {
    if (pic_n == null) pic_n = spoilerPicNum()
    return `/imgs/spoiler/${pad(pic_n, SPOILER_ZEROS)}.${SPOILER_IMGS[pic_n - 1]}`
}


const NO_OP_ZEROS = 2
const NO_OP_IMGS = ['jpg']
export function noOpPicNum() {
    let pic_n = rand_i(1, NO_OP_IMGS.length)
    return pic_n
}
export function noOpPicPath(pic_n = null) {
    if (pic_n == null) pic_n = spoilerPicNum()
    return `/imgs/p_no_op/${pad(pic_n, NO_OP_ZEROS)}.${NO_OP_IMGS[pic_n - 1]}`
}