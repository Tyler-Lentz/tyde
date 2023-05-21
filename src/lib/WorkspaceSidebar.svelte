<script lang="ts">
    import type {TDir} from "../file";
    import WorkspaceDirectory from "./WorkspaceDirectory.svelte";
    import {root} from "../stores";
	import { onDestroy } from "svelte";

    let directory: TDir | null;
    let unsubscribe = root.subscribe((val) => {
        directory = val; 
    });
    onDestroy(() => unsubscribe());

</script>


<div class="first">
<div class="second">
    <WorkspaceDirectory
        bind:directory
        depth={''}
        >
    </WorkspaceDirectory>
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