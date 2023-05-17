<script>
    export let dialog;
    import { slide } from 'svelte/transition';
</script>

<dialog transition:slide bind:this={dialog} on:close>
    <slot/>
</dialog>

<style>
    dialog {
        width: 100%;
        height: 80%;
        border: 1px solid var(--dark-highlight-color);
        background-color: var(--dark-bg-color)
    }
</style>