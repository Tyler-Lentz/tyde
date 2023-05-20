<script lang="ts">
    import { files } from '../stores';

    export let current_file: number | null;

    function handleClick(event: Event) {
        if (event.target instanceof HTMLLIElement) {
            current_file = event.target.dataset.selected === 'true' ? null : parseInt(event.target.dataset.index|| '0');
        }
    }
</script>

<ol>
    {#each $files as file, i}
        <li on:click={handleClick} data-index={i} data-selected={current_file === i}>{file.name}</li>
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
        min-height: 1.3rem;
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
        color: var(--text-highlight-color);
        padding: 0.125rem;
    }

    li:hover, li:focus, li[data-selected="true"] {
        background-color: var(--dark-highlight-color);
    }
</style>