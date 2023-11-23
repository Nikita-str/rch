import { IS_SAFARI } from "../fns"

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

        var canvas = document.createElement('canvas')
        canvas.width = w
        canvas.height = h
        let ctx = canvas.getContext("2d")
        ctx.drawImage(img, 0, 0, w, h)


        let compress_ty = IS_SAFARI ? "image/jpeg" : "image/webp"
        this.compressed = canvas.toDataURL(compress_ty, 0.7) // webp DON't work in Safari?? 
        this.compress_ty = IS_SAFARI ? "jpeg" : "webp"
    }
}
