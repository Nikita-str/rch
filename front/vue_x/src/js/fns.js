
/** 
 * return integer random from `from` to `to` value
 * @param from min value that can be returned
 * @param to max value that can be returned
 * @return 
 * if `to` < `from` => `null`\
 * otherwise => value [`from`; `to`] (both inclusive)
*/
export function i_rand(from, to) {
    if (to < from) return null;
    if (to == from) return from;

    let r_float = Math.random() * (to - from + 1)
    let r_int = Math.floor(r_float)
    return from + r_int
}

export function pad(obj, align_len, align_str = '0') {
    return String(obj).padStart(align_len, align_str)
}
