export class FNode {
    dir: TDir | undefined;
    file: TFile | undefined;

    constructor(obj: any) {
        this.dir = undefined;
        this.file = undefined;

        if ("Dir" in obj) {
            let contents: Array<FNode> = [];

            for (const fnode of obj.Dir.contents) {
                contents.push(new FNode(fnode));
            }

            this.dir = new TDir(obj.Dir.path, contents);
        } else if ("File" in obj) {
            this.file = new TFile(obj.File.path, obj.File.content);
        }
    }

    getPath():string {
        if (this.dir !== undefined) {
            return this.dir.path;
        } else if (this.file !== undefined) {
            return this.file.path;
        }

        return "invalid file";
    }

    getName():string {
        if (this.dir !== undefined) {
            return this.dir.name;
        } else if (this.file !== undefined) {
            return this.file.name;
        }

        return "invalid file";
    }

    isOpen():boolean {
        return false;
    }

    isDir():boolean {
        return this.dir !== undefined;
    }

    isFile():boolean {
        return this.file !== undefined;
    }
}

export class TDir {
    path: string;
    name: string;
    contents: Array<FNode>;
    open: boolean;
    constructor(path: string, contents: Array<FNode>) {
        console.log("making dir", path, contents);
        this.path = path;
        this.name = path.split('/').at(-1) || path;
        this.contents = contents;
        this.open = false;
    }

    isOpen():boolean {
        return this.open;
    }

    toggleOpen() {
        this.open = !this.open;
    }
}

export class TFile {
    path: string;
    name: string;
    content: string | null;
    constructor(path: string, content: string | null) {
        console.log("making file", path, content);
        this.path = path;
        this.name = path.split('/').at(-1) || path;
        this.content = content;
    }

    isOpen():boolean {
        return this.content !== null;
    }
}