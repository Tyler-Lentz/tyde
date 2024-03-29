<script lang="ts">
    import Icon from "./Icon.svelte"
    import type {TDir, TFile, FNode} from "../file"
    import {econsole, opened_files, curr_file} from "../stores"
    import { invoke } from '@tauri-apps/api/tauri'
	import { onDestroy } from "svelte";

    export let directory: TDir | null;

    export let depth: string;

    export function toggle() {
        if (directory !== null) {
            directory.toggleOpen();
            directory = directory;
        }
    }

    let icon_name: string;
    $: icon_name = directory !== null && directory.isOpen() ? "open_dir" : "close_dir";

    function openFile(file: TFile | undefined) {
        if (file === undefined) {
            return;
        }
            
        if ($opened_files.find(f => f.path === file.path)) {
            // file is already opened so dont reopen it
            // either open/close depending on if already open

            if ($curr_file == file) {
                curr_file.set(null);
            } else {
                curr_file.set(file);
            }
            return;
        }

        invoke('open_file', { path: file.path }).then((resp: any) => {

            econsole.set(`Opened ${resp.File.path}`);
            file.setContent(resp.File.content);

            opened_files.update((ofiles: Array<TFile>) => {
                ofiles.push(file);
                econsole.set(`Opened ${file.path}`);
                return ofiles;
            });
        });
    }

    let unsub = curr_file.subscribe((_) => {
        directory = directory;
    });
    onDestroy(() => {
        unsub();
    })
</script>


<div>
    {#if directory !== null}
        <pre on:click={toggle} class="dir">{depth}<Icon bind:name={icon_name}></Icon>{directory.name}</pre>
        {#if directory.isOpen()}
            <div >
                {#each directory.contents as subnode}
                    {#if subnode.isDir()}
                        <svelte:self
                            bind:directory={subnode.dir}
                            depth={depth+'  '}
                            >
                        </svelte:self>
                    {:else}
                        <div>
                            {#if subnode.file !== undefined}
                                <pre 
                                    on:click={() => openFile(subnode.file)} 
                                    class="file"
                                    data-opened={$opened_files.includes(subnode.file)}
                                    data-current={subnode.file === $curr_file}
                                    data-mutated={subnode.file.mutated}
                                    >{depth + '  ' + subnode.file?.name}</pre>
                            {/if}
                        </div>
                    {/if}
                {/each}
            </div>
        {/if}
    {/if}
</div>


<style>
    pre.dir {
        color: var(--text-highlight-color);
    }

    pre.file {
        color: var(--text-default-color);
    }

    pre {
        margin: 0;
        padding: 5px;
        width: 100%;
    }

    pre:hover, pre[data-current="true"] {
        background-color: var(--medium-bg-color);
    }

    pre[data-opened="true"]::after {
        content: "*";
    }

    pre[data-mutated="true"]::after {
        color: var(--warning-color);
    }

    pre[data-mutated="false"]::after {
        color: var(--text-highlight-color);
    }

    div {
        display: flex;
        flex-direction: column;
        width: 100%;
        user-select: none;
        -webkit-user-select: none;
        user-select: none;
        padding-right: 5px;
    }

</style>