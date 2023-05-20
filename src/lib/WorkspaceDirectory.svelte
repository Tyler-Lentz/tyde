<script lang="ts">
    import type {FileSystemNode} from "../defs"

    export let name: String;
    export let subnodes: Array<FileSystemNode>;
</script>

<div>
    <p class="dir">{name}</p>
    <ol>
        {#each subnodes as subnode}
            {#if subnode.isDirectory()}
                <svelte:self
                    name={subnode.name} 
                    subnodes={subnode.subnodes()}
                    >
                </svelte:self>
            {:else}
                <div>
                    <p class="file">{subnode.name}</p>
                </div>
            {/if}
        {/each}
    </ol>
</div>

<style>
    p.dir::before {
        content: ">";
    }

    p.dir {
        color: var(--text-highlight-color);
    }

    p.file {
        color: var(--text-default-color);
    }

    p {
        margin: 0;
        margin-bottom: 5px;
        -width: max-content;
        width: 100%;
        outline: 1px solid red;
    }

    p:hover {
        background-color: var(--medium-bg-color);
    }

    div, ol {
        display: flex;
        flex-direction: column;
    }

    ol {
        list-style-type: none;
        padding-left: 1rem;
        padding-right: 1rem;
        margin-top: 0;
        outline: 1px solid blue;
    }

    div {
        width: 100%;
    }

</style>