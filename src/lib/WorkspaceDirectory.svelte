<script lang="ts">
    import type {FileSystemNode} from "../defs"
    import WorkspaceFile from "./WorkspaceFile.svelte";

    export let name: String;
    export let subnodes: Array<FileSystemNode>;
    export let indent: number;
</script>

<div>
    <p>{name}</p>
    <ol>
        {#each subnodes as subnode}
            {#if subnode.isDirectory()}
                <svelte:self
                    name={subnode.name} 
                    subnodes={subnode.subnodes()}
                    indent={indent+1}>
                </svelte:self>
            {:else}
                <WorkspaceFile
                    name={subnode.name}
                    >
                </WorkspaceFile>
            {/if}
        {/each}
    </ol>
</div>

<style>
    p::before {
        content: ">";
    }

    p {
        margin: 0;
        margin-bottom: 5px;
        color: var(--highlight-color);
    }

    div, ol {
        display: flex;
        flex-direction: column;
    }

    ol {
        list-style-type: none;
        padding-left: 1rem;
        margin-top: 0;
    }

</style>