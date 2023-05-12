export class File {
    name: String;
    content: String;
    absPath: String;
    constructor(name: String, content: String) {
        this.name = name.split('/').at(-1) || name;
        this.absPath = name;
        this.content = content;
    }
}