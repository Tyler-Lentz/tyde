<script lang="ts">
    import type {FileSystemNode} from "../defs"

    export let name: String;
    export let subnodes: Array<FileSystemNode>;
    export let depth: string;
</script>

<div>
    <pre class="dir">{depth + '>' + name}</pre>
    <div>
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
    }

</style>