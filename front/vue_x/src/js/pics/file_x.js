import { IS_SAFARI, IS_FIREFOX } from "../fns"

export class FileX {
    constructor(file) {
        this.file = file
        this.size = file.size
        this.name = file.name
        this.url = URL.createObjectURL(file)
    }

    async compress(canvas, min_dim_sz, max_dim_sz) {
        var img = new Image
        img.src = this.url
        await img.decode()

        this.orig_w = img.width
        this.orig_h = img.height

        let w = img.width
        let h = img.height
        if (w < 1) { w = 1 }
        if (h < 1) { h = 1 }
        let k = 1
        
        if (w > h) {
            if (w > max_dim_sz) k = max_dim_sz / w
        } else {
            if (h > max_dim_sz) k = max_dim_sz / h
        }

        w = Math.round(w * k)
        h = Math.round(h * k)

        if (w < min_dim_sz) w = min_dim_sz
        if (h < min_dim_sz) h = min_dim_sz

        canvas.width = w
        canvas.height = h
        let ctx = canvas.getContext("2d")
        ctx.imageSmoothingEnabled = true
        ctx.imageSmoothingQuality = "high" // not work in Firefox :(
        
        if (IS_FIREFOX) {
            const canvas_big = document.createElement('canvas')
            const ctx_big = canvas_big.getContext('2d')
            canvas_big.width = img.width
            canvas_big.height = img.height
            const blur_size = (canvas_big.width / canvas.width) >> 1
            ctx_big.filter = `blur(${blur_size}px)`
            ctx_big.drawImage(img, 0, 0)

            ctx.drawImage(canvas_big, 0, 0, img.width, img.height, 0, 0, w, h)
        } else {
            ctx.drawImage(img, 0, 0, w, h)
        }
        
        let compress_ty = IS_SAFARI ? "image/jpeg" : "image/webp"
        this.compressed = canvas.toDataURL(compress_ty, 0.7) // webp DON't work in Safari?? 
        this.compress_ty = IS_SAFARI ? "jpeg" : "webp"
    }

    add_base64() {
        new Promise((_, reject) => {
            const reader = new FileReader();
            reader.readAsDataURL(this.file);
            reader.onload = () => {
                this.base64 = reader.result;
            }
            reader.onerror = reject;
        })
    }

    to_post_img() {
        let spoiler = this.spoiler ? true : false
        return {
            file: this.base64.substring(this.base64.indexOf(',') + 1),
            compressed_file: spoiler ? '' : this.compressed.substring(this.compressed.indexOf(',') + 1),
            name: this.name,
            orig_w: this.orig_w,
            orig_h: this.orig_h,
            spoiler,
        }
    }
}

export function img_ext_abbr(c) {
    if (c == 'j') { return 'jpg' }
    if (c == 'p') { return 'png' }
    if (c == 'w') { return 'webp' }
    return null
}