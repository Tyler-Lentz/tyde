<script lang="ts">
    export let init_content: string;
    export let line_number: number;

    export let handleClick: (event: MouseEvent) => void;
    export let handleFocus: (event: FocusEvent) => void;
    export let handleKeyDown: (event: KeyboardEvent) => void;

    export let innerText: string = "";

    export let command_mode: boolean = false;

    export let longest_lineno_len: number;

    let editable: HTMLPreElement;

    export function getLine() {
        return editable;
    }

    function formatLineNumber():string {
        let line_num_str = String(line_number);
        let diff = longest_lineno_len - line_num_str.length;
        let padding = " ".repeat(diff);
        return line_num_str + padding;
    }
</script>

<div class="line-container">
    <span class="line-number"><pre>{formatLineNumber()}</pre></span>
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
        border-right: 1px solid var(--dark-highlight-color);
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