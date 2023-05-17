<script lang="ts">
    import { appWindow } from '@tauri-apps/api/window';
    import { invoke } from '@tauri-apps/api/tauri'
    import Sidebar from './Sidebar.svelte';
    import TextEditor from './TextEditor.svelte';
    import EditorConsole from './EditorConsole.svelte';
    import { File } from '../defs';
    import { files } from '../stores';

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
</script>

<div>
    <Sidebar bind:current_file/> 
    {#if current_file !== null}
        <TextEditor file={$files[current_file]} />
    {/if}
    <EditorConsole bind:this={econsole}/>
</div>

<style>
    :global(:root) {
        --text-default-color: rgb(204,204,198);
        --text-highlight-color: rgb(115, 170, 195);
        --darkest-bg-color: rgb(30,30,30);
        --dark-bg-color: rgb(37,37,38);
        --medium-bg-color: rgb(51,51,51);
        --highlight-color: rgb(179,143,80);
        --dark-highlight-color: rgb(99,112,112);
        --warning-color: rgb(180, 58, 58);
        --font-size: 12pt;
    }
    div {
        background-color: var(--darkest-bg-color);
        display: flex;
        flex-direction: column;
        justify-content: flex-start;
        align-items: flex-start;
        width: 100%;
        height: 100%;
    }
</style>