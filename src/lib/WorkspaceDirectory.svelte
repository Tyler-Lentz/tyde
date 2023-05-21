<script lang="ts">
    import Icon from "./Icon.svelte"
    import type {TDir, TFile, FNode} from "../file"
    import {files, econsole} from "../stores"
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

    function openFile(fnode: TFile | undefined) {
        if (fnode === undefined) {
            return;
        }
        invoke('open_file', { fname: fnode.path }).then((file: any) => {
            econsole.set(`Opened ${file.path}`);
            files.update((files) => {
                fnode.content = file.content;
                files.push(fnode);
                return files;
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
                            bind:directory={subnode}
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