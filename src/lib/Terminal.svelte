<script lang="ts">
    import { invoke } from '@tauri-apps/api/tauri'

    let text: Array<String> = [];
    let input: HTMLInputElement;

    function focus() {
        input.focus();
    }

    function checkForSubmission(event: KeyboardEvent) {
        console.log(event.key);
        if (event.key === "Enter") {
            console.log("in enter");
            let command = input.value;
            invoke('shell_exec', { command: command }).then((output: any) => {
                console.log("outpt")
                console.log(output);
                text.push(`${command}${output}`);
            });
        }
    }

</script>

<div on:click={focus}>
    {#each text as line}
        <pre>{line}</pre> 
    {/each}
    <span><input bind:this={input} type="text"/></span>
</div>
<svelte:window on:keydown={checkForSubmission}></svelte:window>

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
        user-select: none;
        -webkit-user-select: none;
        font-size: var(--font-size);
    }

    input {
        all: unset;
        font-family: monospace;
        width: 95%;
    }

    pre, input {
        padding: 0;
        margin: 0;
        user-select: text;
        -webkit-user-select: text;
    }

    pre::before, span::before{
        content: "$: ";
        font-family: monospace;
        color: var(--text-highlight-color);
    }

    pre::selection, input::selection {
        background-color: var(--text-highlight-color);
        color: var(--text-highlight-color);
    }
</style>