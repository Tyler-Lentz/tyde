<script lang="ts">
    import { appWindow } from '@tauri-apps/api/window';
    import { invoke } from '@tauri-apps/api/tauri'
    import Topbar from './Topbar.svelte';
    import TextEditor from './TextEditor.svelte';
    import EditorConsole from './EditorConsole.svelte';
    import { Directory, File } from '../defs';
    import type {FileSystemNode} from '../defs'
    import { files } from '../stores';
	import WorkspaceSidebar from './WorkspaceSidebar.svelte';

    let econsole: EditorConsole;

    let current_file: number | null = null;

    appWindow.listen('open-file', (event: any) => {
        files.update((files) => {
            let file = new File(event.payload.name, event.payload.content);

            for (const f of files) {
                if (f.absPath === file.absPath) {
                    econsole.add(`File ${file.absPath} already open`);
                    return files;
                }
            }

            files.push(file);
            current_file = files.length - 1;
            econsole.add(`Opened ${event.payload.name}`);
            return files;
        });
    });

    appWindow.listen('save-file', (_) => {
        if (current_file !== null) {
            let file = $files[current_file];
            if (file.absPath === "") {
                econsole.add(`Cannot save unnamed file`);
                return;
            }

            invoke('save_file', { fpath: file.absPath, content: file.content }).then((_) => {
                econsole.add(`Saved ${file.absPath}`)
            });
        }
    });

    appWindow.listen('new-file', (_) => {
        files.update((files) => {
            let file = new File("", "");
            files.push(file);
            current_file = files.length - 1;
            econsole.add(`Created new file`);
            return files;
        });
    });


    let directory: Directory | null = null;

    let empty_file = new File("null", null);

    appWindow.listen('open-directory', (msg: any) => {
        let root = msg.payload.root.Dir;

        function traverse(curr_dir: any):Directory{
            let name: string = curr_dir[0];
            let children: Array<any> = curr_dir[1];

            let output_children: Array<FileSystemNode> = [];
            for (const child of children) {
                if ("File" in child) {
                    output_children.push(new File(child.File, null));
                } else {
                    let child_node = traverse(child.Dir);
                    output_children.push(child_node);
                }
            }
            return new Directory(name, output_children, false);
        }

        directory = traverse(root);
    });
    
</script>

<div>
    <Topbar bind:current_file/> 
    <main>
        <WorkspaceSidebar bind:directory></WorkspaceSidebar> 
        {#if current_file !== null}
            <TextEditor file={$files[current_file]} />
        {:else}
            <TextEditor file={empty_file} />
        {/if}
    </main>
    <EditorConsole bind:this={econsole}/>
</div>

<style>
    div {
        background-color: var(--darkest-bg-color);
        display: flex;
        flex-direction: column;
        justify-content: flex-start;
        align-items: flex-start;
        width: 100%;
        height: 100%;
    }
    main {
        display: flex;
        flex-direction: row;
        justify-content: flex-start;
        align-items: flex-start;
        width: 100%;
        height: 100%;
    }
</style>