export class FileSystemNode {
    name: String;
    absPath: String;

    constructor(absPath: String) {
        if (absPath == "") {
            this.name = "<unnamed>"
        } else {
            this.name = absPath.split('/').at(-1) || absPath;
        }

        this.absPath = absPath;
    }

    isDirectory():boolean {
        return false;
    }

    subnodes():Array<FileSystemNode> {
        return [];
    }
}

export class File extends FileSystemNode {
    content: String;
    constructor(absPath: String, content: String) {
        super(absPath);
        this.content = content;
    }
}

export class Directory extends FileSystemNode {
    content: Array<FileSystemNode>;
    constructor(absPath: String, content: Array<FileSystemNode>) {
        super(absPath);
        this.content = content;
    }

    isDirectory():boolean {
        return true;
    }

    subnodes():Array<FileSystemNode> {
        return this.content;
    }
}