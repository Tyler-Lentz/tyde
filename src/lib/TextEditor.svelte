<script lang="ts">
	import { onDestroy } from "svelte";
    import {curr_file} from "../stores"
    import {arrowLeft, arrowRight, setIndex, getIndex} from '../util';

    let contents: Array<[string, string]> = [];
    let longest_lineno_len: number;

    let unsub = curr_file.subscribe((new_file) => {
        if (new_file !== null && new_file.content !== null) {
            let split_lines = new_file.content.split('\n');
            longest_lineno_len = String(split_lines.length).length;
            contents = split_lines.map((line_content, line_number) => [formatLineNumber(line_number + 1), line_content])
        } else {
            contents = [];
            longest_lineno_len = 0;
        }
    });

    function formatLineNumber(line_number: number):string {
        return String(line_number).padEnd(longest_lineno_len, ' ');
    }

    onDestroy(() => {
        unsub();
    });

    let command_mode: boolean = false;
    let selected_line: number = 0;
    let line_elems: Array<HTMLPreElement> = [];
    
    let farthest_index: number = 0;

    function handleClick(event: MouseEvent) {
        if (event.target !== null) {
            (event.target as HTMLPreElement)?.focus();
        }
    }

    function handleFocus(event: FocusEvent) {
        if (event.target !== null) {
            selected_line = line_elems.indexOf(event.target as HTMLPreElement);
        }
    }

    function handleKeyDown(event: KeyboardEvent) {
        if (command_mode) {
            handleKeyDownCommandMode(event);
        } else {
            handleKeyDownNormalMode(event);
        }
    }

    function handleKeyDownNormalMode(event: KeyboardEvent) {
        switch (event.key) {
            case "Escape":
                event.preventDefault();
                command_mode = true;
                return;
            case "ArrowDown":
                event.preventDefault();
                if (selected_line < contents.length - 1) {
                    selected_line++;
                    line_elems[selected_line].focus();
                    setIndex(farthest_index, line_elems[selected_line]);
                }
                break;
            case "ArrowUp":
                event.preventDefault();
                if (selected_line > 0) {
                    selected_line--;
                    line_elems[selected_line].focus();
                    setIndex(farthest_index, line_elems[selected_line]);
                }
                break;
            case "ArrowRight":
                event.preventDefault();
                arrowRight(line_elems[selected_line]);
                farthest_index = getIndex();
                break;
            case "ArrowLeft":
                event.preventDefault();
                arrowLeft(line_elems[selected_line]);
                farthest_index = getIndex();
                break;
        }
    }

    function handleKeyDownCommandMode(event: KeyboardEvent) {
        event.preventDefault();
        switch (event.key) {
            case "ArrowDown":
            case "j":
                if (selected_line < contents.length - 1) {
                    selected_line++;
                    line_elems[selected_line].focus();
                    setIndex(farthest_index, line_elems[selected_line]);
                }
                break;
            case "ArrowUp":
            case "k":
                if (selected_line > 0) {
                    selected_line--;
                    line_elems[selected_line].focus();
                    setIndex(farthest_index, line_elems[selected_line]);
                }
                break;
            case "ArrowRight":
            case "l":
                arrowRight(line_elems[selected_line]);
                farthest_index = getIndex();
                break;
            case "ArrowLeft":
            case "h":
                arrowLeft(line_elems[selected_line]);
                farthest_index = getIndex();
                break;
            case "i":
                command_mode = false;
                break;
        }
    }
</script>

<div class="container">
    {#each contents as [line_number, line_content], index}
    <div class="line-container">
        <span class="linenum"><pre>{line_number}</pre></span>
        <pre 
            bind:this={line_elems[index]} 
            data-command-mode={command_mode}
            on:keydown={handleKeyDown} 
            on:click={handleClick}
            on:focus={handleFocus}
            contenteditable
            >{line_content}</pre>
    </div>
    {/each}
</div>

<style >
    div.container {
        display: flex;
        flex-direction: column;
        width: 100%;
        height: 100%;
        margin: 0;
        background-color: var(--dark-bg-color);
        color: var(--text-default-color);
        font-size: var(--font-size);
        height: var(--text-editor-height);
        overflow: scroll;
    }

    div.line-container {
        width: 100%;
    }

    div.line-container:has(pre[contenteditable]:focus) {
        border-top: 1px solid var(--opaque-border);
        border-bottom: 1px solid var(--opaque-border);
    }

    span.linenum {
        display: inline;
        padding:0;
        margin:0;
        margin-right:0.5rem;
        user-select: none;
        -webkit-user-select: none;
        color: var(--text-highlight-color);
        font-family: monospace;
        padding-right: 0.75rem;
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
