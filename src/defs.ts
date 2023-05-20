export class FileSystemNode {
    name: string;
    absPath: string;

    constructor(absPath: string) {
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

    isOpen():boolean {
        return false;
    }

    subnodes():Array<FileSystemNode> {
        return [];
    }
}

export class File extends FileSystemNode {
    content: string | null;
    constructor(absPath: string, content: string | null) {
        super(absPath);
        this.content = content;
    }

    isOpen():boolean {
        return this.content !== null;
    }
}

export class Directory extends FileSystemNode {
    content: Array<FileSystemNode>;
    open: boolean;
    constructor(absPath: string, content: Array<FileSystemNode>, open: boolean) {
        super(absPath);
        this.content = content;
        this.open = open;
    }

    isDirectory():boolean {
        return true;
    }

    isOpen():boolean {
        return this.open;
    }

    subnodes():Array<FileSystemNode> {
        return this.content;
    }
}