export class FNode {
    dir: TDir | undefined;
    file: TFile | undefined;

    #searchmap: Map<String, TFile>;

    constructor(obj: any, map: Map<String, TFile> | undefined) {
        this.dir = undefined;
        this.file = undefined;

        if (map === undefined) {
            this.#searchmap = new Map();
        } else {
            this.#searchmap = map;
        }

        if ("Dir" in obj) {
            let contents: Array<FNode> = [];

            for (const fnode of obj.Dir.contents) {
                contents.push(new FNode(fnode, this.#searchmap));
            }

            this.dir = new TDir(obj.Dir.path, contents);
        } else if ("File" in obj) {
            this.file = new TFile(obj.File.path, obj.File.content);
            this.#searchmap.set(this.file.path, this.file);
        }
    }

    search(path: string):TFile|undefined {
        return this.#searchmap.get(path);
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
    content: Array<string> | null;
    mutated: boolean;
    constructor(path: string, content: string | null) {
        this.path = path;
        this.name = path.split('/').at(-1) || path;
        if (content !== null) {
            this.content = content.split('\n');
        } else {
            this.content = null;
        }
        console.log(this.content);
        this.mutated = false;
    }

    setContent(content: string) {
        this.content = content.split('\n');
    }

    isOpen():boolean {
        return this.content !== null;
    }
}