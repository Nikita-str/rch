
/** 
 * return integer random from `from` to `to` value
 * @param from min value that can be returned
 * @param to max value that can be returned
 * @return 
 * if `to` < `from` => `null`\
 * otherwise => value [`from`; `to`] (both inclusive)
*/
export function rand_i(from, to) {
    if (to < from) return null;
    if (to == from) return from;

    let r_float = Math.random() * (to - from + 1)
    let r_int = Math.floor(r_float)
    return from + r_int
}

export function pad(obj, align_len, align_str = '0') {
    return String(obj).padStart(align_len, align_str)
}

export function rand_n_arr(n, shift = 0) {
    let arr = Array.from({length: n}, (_, i) => i + shift);
    
    for (let ind = n - 1; ind > 0; ind--) {
        const ind_swap = rand_i(0, ind);
        [arr[ind], arr[ind_swap]] = [arr[ind_swap], arr[ind]];
      }
    return arr
}

export function trim(s, c) {
    let start = 0; 
    let end = s.length;

    while(start < end && s[start] === c) start++;
    while(end > start && s[end - 1] === c) end--;

    return s.substring(start, end);
}



/** 
 * @param obj object with $route
 * @return boardUrl
*/
export function boardUrlCalc(obj) {
    var path = (typeof(obj) == 'string') ? obj : obj.$route.path;
    return trim(path, "/").split('/')[0]
}

export const IS_SAFARI = navigator.userAgent.toLowerCase().indexOf("Safari") > -1
