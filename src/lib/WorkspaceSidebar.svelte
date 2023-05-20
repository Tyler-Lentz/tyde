<script lang="ts">
    import type {Directory} from "../defs";
    import WorkspaceDirectory from "./WorkspaceDirectory.svelte";

    export let directory: Directory | null;
</script>


<div class="first">
<div class="second">
    {#if directory !== null}
        <WorkspaceDirectory
            bind:directory
            depth={''}
            >
        </WorkspaceDirectory>
    {:else}
        <WorkspaceDirectory 
            directory={null}
            depth={''}
            >
        </WorkspaceDirectory>
    {/if}
</div>
</div>

<style>
    div.second{
        overflow-y:scroll;
        overflow-x:hidden;
        height: 100%;
    }
    div.first{
        display: flex;
        flex-direction: column;
        border: 1px solid var(--dark-highlight-color);
        background-color: var(--dark-bg-color);
        height: calc(90vh + 3px);
        border-bottom: none;
        border-top: none;
    }
</style>