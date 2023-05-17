export class File {
    name: String;
    content: String;
    absPath: String;
    constructor(absPath: String, content: String) {
        if (absPath === "") {
            this.name = "<unnamed>";
            this.absPath = "";
            this.content = content;
        } else {
            this.name = absPath.split('/').at(-1) || absPath;
            this.absPath = absPath;
            this.content = content;
        }

    }
}