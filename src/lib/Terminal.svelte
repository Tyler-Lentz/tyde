<script lang="ts">

    let text: Array<String>;
    text = [
        "line 1 line 1",
        "line 2 line 2",
        "cat route/to/file"
    ];
</script>

<div>
    {#each text as line}
        <pre>{line}</pre> 
    {/each}
</div>

<style>
    div {
        background-color: var(--dark-bg-color);
        color: var(--text-default-color);
        border: 1px solid var(--dark-highlight-color);
        border-left: none;
        border-bottom: none;
        box-sizing: border-box;
        height: 15vh;
        width: 100%;
    }

    pre::before{
        content: "$: ";
        font-family: monospace;
        color: var(--text-highlight-color);
    }
</style>