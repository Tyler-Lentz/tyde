<script lang="ts">
    import type {Directory} from "../defs"
    import WorkspaceDirectory from "./WorkspaceDirectory.svelte";

    export let directory: Directory | null;
</script>

<div >
    {#if directory !== null}
        <WorkspaceDirectory
            name={directory.name} 
            subnodes={directory.subnodes()}
            depth={''}
            >
        </WorkspaceDirectory>
    {:else}
        <WorkspaceDirectory 
            name="" 
            subnodes={[]}
            depth={''}
            >
        </WorkspaceDirectory>
    {/if}
</div>

<style>
    div {
        display: flex;
        flex-direction: column;
        border: 1px solid var(--dark-highlight-color);
        background-color: var(--dark-bg-color);
        height: calc(90vh + 3px);
        width: max-content;
        border-bottom: none;
        border-top: none;
        padding-right: 10px;
    }
</style>