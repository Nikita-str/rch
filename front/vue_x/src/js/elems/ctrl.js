import { sha3_256 } from 'js-sha3';
import { notific_ctor, NOTIFIC_TY_ERR } from "@/js/elems/notific";

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
export function notific_ctor_err_ctrl(msg) {
    // return notific_ctor(NOTIFIC_TY_ERR, msg, top = true, left = true)
    return notific_ctor(NOTIFIC_TY_ERR, msg, 2_000, true, true)
}