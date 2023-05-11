export class File {
    name: String;
    content: String;
    constructor(name: String, content: String) {
        this.name = name.split('/').at(-1) || name;
        this.content = content;
    }
}