export class FileX {
    constructor(file) {
        this.file = file
        this.size = file.size
        this.name = file.name
        this.url = URL.createObjectURL(file)
    }
}
