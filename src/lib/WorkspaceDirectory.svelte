<script lang="ts">
    import Icon from "./Icon.svelte"
    import type {TDir, TFile, FNode} from "../file"
    import {econsole, opened_files} from "../stores"
    import { invoke } from '@tauri-apps/api/tauri'

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
            return
        }

        invoke('open_file', { path: file.path }).then((resp: any) => {

            econsole.set(`Opened ${resp.File.path}`);
            file.content = resp.File.content;

            opened_files.update((ofiles: Array<TFile>) => {
                ofiles.push(file);
                econsole.set(`Opened ${file.path}`);
                return ofiles;
            });
        });
    }
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
                            <pre on:click={() => openFile(subnode.file)} class="file">{depth + '  ' + subnode.file?.name}</pre>
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

    pre:hover {
        background-color: var(--medium-bg-color);
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