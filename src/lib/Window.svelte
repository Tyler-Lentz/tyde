<script lang="ts">
    import { appWindow } from '@tauri-apps/api/window';
    import Sidebar from './Sidebar.svelte';
    import TextEditor from './TextEditor.svelte';
    import { File } from '../defs';
    import { files } from '../stores';

    let current_file: number | null = null;
    appWindow.listen('open-file', (event: any) => {
        files.update((files) => {
            files.push(new File(event.payload.name, event.payload.content));
            current_file = files.length - 1;
            return files;
        });
    });
</script>

<div>
    <Sidebar /> 
    {#each $files as file, i}
        {#if current_file === i}
            <TextEditor file={file} />
        {/if}
    {/each}
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
        flex-direction: row;
        justify-content: flex-start;
        align-items: flex-start;
        width: 100%;
        height: 100%;
    }
</style>