import { sha3_256 } from 'js-sha3';
import { notific_ctor, NOTIFIC_TY_ERR, NOTIFIC_TY_INFO } from "@/js/elems/notific";
import { trim } from '@/js/fns.js'

export function cmp_pwd_hash(pwd, inner_part) {
    let pwd_hash = pwd
    if (pwd_hash) {
        pwd_hash = pwd_hash.split(' ')
        if (pwd_hash.length != 2) {
            notific_ctor_err_ctrl("`nonce&pwd` must contain only `nonce` and `pwd` and be splited with space")
            return false
        }
        pwd_hash = pwd_hash[0] + inner_part + pwd_hash[1]
        pwd_hash = sha3_256(pwd_hash)
    } else {
        notific_ctor_err_ctrl("empty `nonce&pwd`")
        return false
    }
    return pwd_hash
}
export function url_prepare(url) {
    if (!url) { 
        notific_ctor_err_ctrl("emtpty url")
        return false
    }
    url = trim(url, '/')
    if (url.length == 0) { 
        notific_ctor_err_ctrl("emtpty url")
        return false
    }
    return url
}
export function positive_num_prepare(value, name) {
    let val = parseInt(value)
    if (!val) { notific_ctor_err_ctrl(`emtpty ${name} count`); return }
    if (val < 1) { notific_ctor_err_ctrl(`${name} must be more than 0`); return }
    return val
}
export function notific_ctor_err_ctrl(msg) {
    // return notific_ctor(NOTIFIC_TY_ERR, msg, top = true, left = true)
    return notific_ctor(NOTIFIC_TY_ERR, msg, 2_000, true, true)
}
export function notific_ctor_ok_ctrl(msg) {
    return notific_ctor(NOTIFIC_TY_INFO, msg, 2_000, true, true)
}