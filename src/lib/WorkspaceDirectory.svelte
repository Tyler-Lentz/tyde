<script lang="ts">
    import Icon from "./Icon.svelte"
    import type {Directory} from "../defs"

    export let directory: Directory | null;

    export let depth: string;

    export function toggle() {
        if (directory !== null) {
            directory.open = !directory.open;
        }
    }

    let icon_name: string;
    $: icon_name = directory !== null && directory.isOpen() ? "open_dir" : "close_dir";
    $: console.log(icon_name);

    function openFile(absPath: string) {
        // TODO: open file
        // invoke('save_file', { fpath: file.absPath, content: file.content }).then((_) => {
        //     econsole.add(`Saved ${file.absPath}`)
        // });
    }
</script>


<div>
    {#if directory !== null}
        <pre on:click={toggle} class="dir">{depth}<Icon bind:name={icon_name}></Icon>{directory.name}</pre>
        {#if directory.isOpen()}
            <div >
                {#each directory.subnodes() as subnode}
                    {#if subnode.isDirectory()}
                        <svelte:self
                            bind:directory={subnode}
                            depth={depth+'  '}
                            >
                        </svelte:self>
                    {:else}
                        <div>
                            <pre class="file">{depth + '  ' + subnode.name}</pre>
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