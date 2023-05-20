<script lang="ts">
    import type {Directory} from "../defs"
    import WorkspaceDirectory from "./WorkspaceDirectory.svelte";
    import WorkspaceFile from "./WorkspaceFile.svelte";

    export let directory: Directory;
</script>

<div>
    <WorkspaceDirectory
        name={directory.name} 
        subnodes={directory.subnodes()}
        indent={0}>
    </WorkspaceDirectory>
</div>

<style>
    div {
        display: flex;
        flex-direction: column;
        border: 1px solid var(--dark-highlight-color);
        background-color: var(--dark-bg-color);
        height: calc(90vh + 3px);
        width: 16vw;
        max-width: 16vw;
        overflow-x: scroll;
        border-bottom: none;
        border-top: none;
        padding-right: 10px;
    }
</style>