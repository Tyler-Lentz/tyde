<script lang="ts">
    import { opened_files, curr_file } from '../stores';
    import type { TFile } from '../file';
	import { onDestroy } from 'svelte';

    function handleClick(event: Event) {
        if (event.target instanceof HTMLLIElement) {
            let path = event.target.dataset.path;
            curr_file.set(event.target.dataset.selected === 'true' ? null : $opened_files.find(file => file.path === path) || null);
        }
    }

    let unsub = curr_file.subscribe((_) => {
        opened_files.update(a => a);
    });
    onDestroy(() => {
        unsub();
    })
</script>

<ol>
    {#each $opened_files as file, i}
        <li 
            on:click={handleClick} 
            data-mutated={file.mutated} 
            data-path={file.path} 
            data-selected={$curr_file === file}
            >
            {#if file.name == ""}
                {"\<unnamed-file\>"}
            {:else}
                {file.name}
            {/if}
        </li>
    {/each}
</ol>

<style>
    ol {
        list-style-type: none;
        display: flex;
        flex-direction: row;
        justify-content: left;
        align-items: flex-start;
        width:auto;
        padding: 0;
        margin: 0;
        background-color: var(--dark-bg-color);
        border-bottom: 1px solid var(--dark-highlight-color);
        width: 100vw;
        min-height: var(--topbar-height);
        -webkit-user-select: none;
        user-select: none;
    }

    li {
        text-align: center;
        overflow-x: hidden;
        min-width: min-content;
        min-height: 1rem;
        width: 10vw;
        white-space: nowrap;
        color: var(--text-default-color);
        padding: 0.125rem;
    }

    li:hover, li:focus, li[data-selected="true"] {
        background-color: var(--medium-bg-color);
    }

    li[data-mutated="true"] {
        color: var(--warning-color);
    }
</style>