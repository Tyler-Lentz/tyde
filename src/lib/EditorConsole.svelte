<script lang="ts">
    export let full: boolean;
    let history: Array<String> = [];

    export function add(message: String) {
        history.push(message);
        history = history;
    }
</script>

{#if full}
    <div>
        {#each history as message}
            <pre>{message}</pre>
        {/each}
    </div>
{:else}
    <pre>{history.at(-1) || ""}</pre>
{/if}

<style>
    pre {
        color: var(--text-highlight-color);
        background-color: var(--dark-bg-color);
        padding: 0.125rem;
        margin: 0;
        padding: 0;
        border-top: 1px solid var(--dark-highlight-color);

        width: 100%;
        height: 1.25rem;
    }

    pre:hover, pre:focus {
        background-color: var(--dark-highlight-color);
    }
</style>