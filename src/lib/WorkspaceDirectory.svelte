<script lang="ts">
    import type {FileSystemNode} from "../defs"
    import Icon from "./Icon.svelte"

    export let name: string;
    export let subnodes: Array<FileSystemNode>;
    export let depth: string;
    let active: boolean = true;

    export function toggle() {
        active = !active;
    }

    let icon_name: string;
    $: icon_name = active ? "open_dir" : "close_dir";
    $: console.log(icon_name);
</script>


<div>
    <pre on:click={toggle} class="dir">{depth}<Icon bind:name={icon_name}></Icon>{name}</pre>
    {#if active}
        <div >
            {#each subnodes as subnode}
                {#if subnode.isDirectory()}
                    <svelte:self
                        name={subnode.name} 
                        subnodes={subnode.subnodes()}
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
</div>

<style>
    svg {
        width: 1rem;
        height: 1rem;
    }
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