<script lang="ts">
    import { appWindow } from '@tauri-apps/api/window';
    import { invoke } from '@tauri-apps/api/tauri'
    import Sidebar from './Sidebar.svelte';
    import TextEditor from './TextEditor.svelte';
    import EditorConsole from './EditorConsole.svelte';
    import { File } from '../defs';
    import { files } from '../stores';

    let console: EditorConsole;

    let current_file: number | null = null;

    appWindow.listen('open-file', (event: any) => {
        files.update((files) => {
            let file = new File(event.payload.name, event.payload.content);

            for (const f of files) {
                if (f.absPath === file.absPath) {
                    console.add(`File ${file.absPath} already open`);
                    return files;
                }
            }

            files.push(file);
            current_file = files.length - 1;
            console.add(`Opened ${event.payload.name}`);
            return files;
        });
    });

    appWindow.listen('save-file', (_) => {
        if (current_file !== null) {
            let file = $files[current_file];
            invoke('save_file', { fpath: file.absPath, content: file.content }).then((_) => {
                console.add(`Saved ${file.absPath}`)
            });
        }
    });
</script>

<div>
    <Sidebar bind:current_file/> 
    {#each $files as file, i}
        {#if current_file === i}
            <TextEditor file={file} />
        {/if}
    {/each}
    <EditorConsole bind:this={console} full={false}/>
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