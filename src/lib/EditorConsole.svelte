<script lang="ts">
    import ScreenDialog from './ScreenDialog.svelte';
    let dialog: HTMLDialogElement;

    let history: Array<String> = [];

    export function add(message: String) {
        history.push(message);
        history = history;
    }

    export function open() {
        dialog.showModal()
    }
</script>

<pre on:click={open}>{history.at(-1) || ""}</pre>
<ScreenDialog bind:dialog>
    {#each history as message}
        <pre style:border="none">{message}</pre>
    {/each}
</ScreenDialog>

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