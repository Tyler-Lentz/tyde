<script lang="ts">
    export let name: String;
</script>

<div>
    <slot/>
</div>

<style>
    div {
        color: var(--text-highlight-color);
        background-color: var(--dark-bg-color);
    }
</style>