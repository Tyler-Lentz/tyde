<script lang="ts">
    import { appWindow } from '@tauri-apps/api/window';
    import { invoke } from '@tauri-apps/api/tauri'
    import Topbar from './Topbar.svelte';
    import TextEditor from './TextEditor.svelte';
    import EditorConsole from './EditorConsole.svelte';
    import { TDir, TFile } from '../file';
    import {FNode} from '../file'
    import { root, opened_files, curr_file } from '../stores';
	import WorkspaceSidebar from './WorkspaceSidebar.svelte';
	import { onDestroy } from 'svelte';

    let econsole: EditorConsole;

    appWindow.listen('open-file', (event: any) => {
        // files.update((files) => {
        //     let file = new TFile(event.payload.name, event.payload.content);

        //     for (const f of files) {
        //         if (f.path === file.path) {
        //             econsole.add(`File ${file.path} already open`);
        //             return files;
        //         }
        //     }

        //     files.push(file);
        //     current_file = files.length - 1;
        //     econsole.add(`Opened ${file.path}`);
        //     return files;
        // });
    });

    appWindow.listen('save-file', (_) => {
        // if (current_file !== null) {
        //     let file = $files[current_file];
        //     if (file.path === "") {
        //         econsole.add(`Cannot save unnamed file`);
        //         return;
        //     }

        //     invoke('save_file', { fpath: file.path, content: file.content }).then((_) => {
        //         econsole.add(`Saved ${file.path}`)
        //     });
        // }
    });

    appWindow.listen('new-file', (_) => {
        // files.update((files) => {
        //     let file = new TFile("", "");
        //     files.push(file);
        //     current_file = files.length - 1;
        //     econsole.add(`Created new file`);
        //     return files;
        // });
    });

    let empty_file = new TFile("null", null);

    appWindow.listen('open-directory', (msg: any) => {
        let top = new FNode(msg.payload, undefined);
        if (top.dir === undefined) {
            econsole.add("Error opening directory: no root found.");
            return;
        }
        
        root.set(top.dir);
    });

    let old_num_files: number = 0;

    let unsub = opened_files.subscribe((ofiles) => {
        if (ofiles.length > old_num_files) {
            // New opened file at end of list
            curr_file.set(ofiles.at(-1) || null);
        } else if (ofiles.length < old_num_files) {
            if ($curr_file !== null && !ofiles.includes($curr_file)) {
                // current file was closed
                curr_file.set(null);
            }
        }

        old_num_files = ofiles.length;
    });
    onDestroy(() => {unsub();});

</script>

<div>
    <Topbar /> 
    <main>
        <WorkspaceSidebar></WorkspaceSidebar> 
        {#if $curr_file !== null}
            <TextEditor bind:current_file={$curr_file} />
        {:else}
            <TextEditor current_file={empty_file} />
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