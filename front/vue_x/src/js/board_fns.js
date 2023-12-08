import { trim } from './fns'

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
