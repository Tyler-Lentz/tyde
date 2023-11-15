<script lang="ts">
    import {curr_file} from "../stores"
	import { onDestroy } from "svelte";

    let longest_lineno_len: number = 1;

    let unsub = curr_file.subscribe((file) => {
        if (file !== null && file.content !== null) {
            longest_lineno_len = String(file.content.length).length;
        } else {
            longest_lineno_len = 1;
        }
    });

    onDestroy(() => {
        unsub();
    });

    export let init_content: string;
    export let line_number: number;

    export let handleClick: (event: MouseEvent) => void;
    export let handleFocus: (event: FocusEvent) => void;
    export let handleKeyDown: (event: KeyboardEvent) => void;

    export let innerText: string = "";

    export let command_mode: boolean = false;

    let editable: HTMLPreElement;

    export function getLine() {
        return editable;
    }

</script>

<div class="line-container">
    <span class="line-number"><pre>{line_number}</pre></span>
    <pre 
        class="line-content"
        contenteditable
        bind:innerText
        bind:this={editable}
        data-command-mode={command_mode}
        data-line-number={line_number}
        on:click={handleClick}
        on:focus={handleFocus}
        on:keydown={handleKeyDown}
        >{init_content}</pre>
</div>

<style>
    div.line-container {
        width: 100%;
        display: grid;
        grid-template-rows: auto;
        grid-template-columns: max-content 1fr;
    }

    div.line-container:has(pre[contenteditable]:focus) {
        outline: 1px solid var(--opaque-border);
    }

    span.line-number {
        width: 5ch;
        display: inline;
        padding:0;
        margin:0;
        margin-right:0.5rem;
        user-select: none;
        -webkit-user-select: none;
        color: var(--text-highlight-color);
        font-family: monospace;
        padding-right: 0.25rem;
        padding-left: 0.25rem;
        border-right: 1px solid var(--opaque-border);
    }

    pre {
        display: inline;
        margin: 0;
        padding: 0;
        outline: 0;
    }

    pre[contenteditable] {
        color: var(--text-default-color);
        font: monospace;
        width: 100%;
    }

    pre[data-command-mode="true"] {
        caret-color: var(--text-highlight-color);
    }
</style>