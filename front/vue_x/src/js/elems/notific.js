
export const NOTIFIC_TY_ERR = "err"
export const NOTIFIC_TY_WARN = "warn"
export const NOTIFIC_TY_INFO = "info"

/**
 * @param {String} notific_type error(red), warn(yellow), info(blue)
 * @param {String} msg
 * @param {Number} lifetime_ms
 */
export function notific_ctor(notific_type, msg, lifetime_ms = 2000, top = false, left = false) {
    const MAX_LIFETIME_MS = 8_000
    const HORIZONTAL_SHIFT = "0.6em"
    if (lifetime_ms > MAX_LIFETIME_MS) { lifetime_ms = MAX_LIFETIME_MS }

    let unused_notific_slot = notific_slots.findIndex((x) => !x)
    notific_slots[unused_notific_slot] = 1
    let vertical_shift_em = 0.6 /* first elem indent */ + unused_notific_slot * (0.3 /* gap between notifics */ + 1.5 /* notific h */)

    const notific = document.createElement("div");
    notific.innerText = msg
    notific.classList.add("E-notific")

    if (top) { notific.style.top = vertical_shift_em + "em" } 
    else { notific.style.bottom = vertical_shift_em + "em" }
    
    if (left) { notific.style.left = HORIZONTAL_SHIFT } 
    else { notific.style.right = HORIZONTAL_SHIFT }

    notific.style.background = `var(--E--notific-${notific_type}-bg)`
    notific.style.borderColor = `var(--E--notific-${notific_type}-border)`
    
    const parent = document.getElementById("notific-parent")
    parent.appendChild(notific)
    
    setTimeout(() => {
        notific.remove()
        notific_slots[unused_notific_slot] = null
        // parent.removeChild(notific)
    }, lifetime_ms)
}

var notific_slots = new Array(1_000) // MAX notific is 1k ¯\_(ツ)_/¯ seems enough