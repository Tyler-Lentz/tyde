<script lang="ts">
	import { onDestroy } from "svelte";
    import {curr_file} from "../stores"
    import {arrowLeft, arrowRight, setIndex, getIndex} from '../util';

    let contents: Array<string> = [];
    let longest_lineno_len: number;

    let unsub = curr_file.subscribe((new_file) => {
        if (new_file !== null && new_file.content !== null) {
            longest_lineno_len = String(new_file.content.length).length;
            contents = new_file.content;
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
            farthest_index = getIndex();
        }
    }

    function handleFocus(event: FocusEvent) {
        if (event.target !== null) {
            selected_line = line_elems.indexOf(event.target as HTMLPreElement);
        }
    }

    function handleKeyDown(event: KeyboardEvent) {
        if (command_mode) {
            event.preventDefault();
        }

        switch (event.key) {
            case "Escape": // Enter command mode
                if (!command_mode) {
                    event.preventDefault();
                    command_mode = true;
                    break;
                }
            case "j": // move cursor down if command mode
                if (!command_mode) {
                    break;
                }
            case "ArrowDown": // move cursor down
                if (selected_line < contents.length - 1) {
                    selected_line++;
                    line_elems[selected_line].focus();
                    setIndex(farthest_index, line_elems[selected_line]);
                }
                break;

            case "k": // move cursor up if command mode
                if (!command_mode) {
                    break;
                }
            case "ArrowUp": // move cursor up
                if (selected_line > 0) {
                    selected_line--;
                    line_elems[selected_line].focus();
                    setIndex(farthest_index, line_elems[selected_line]);
                }
                break;

            case "a": // move cursor right and enter insert mode if command mode
                if (!command_mode) {
                    break;
                }
                command_mode = false;
                arrowRight(line_elems[selected_line]);
                farthest_index = getIndex();
                break;

            case "l": // move cursor right if command mode
                if (!command_mode) {
                    break;
                }
            case "ArrowRight": // move cursor right
                arrowRight(line_elems[selected_line]);
                farthest_index = getIndex();
                break;

            case "h": // move cursor left if command mode
                if (!command_mode) {
                    break;
                }
            case "ArrowLeft": // move cursor left
                arrowLeft(line_elems[selected_line]);
                farthest_index = getIndex();
                break;

            case "i": // enter insert mode
                command_mode = false;
                break;

            case "$": // move cursor to end of line
                if (command_mode) {
                    line_elems[selected_line].focus();
                    setIndex(line_elems[selected_line].innerText.length, line_elems[selected_line]);
                    farthest_index = getIndex();
                }
                break;
            case "0": // move cursor to beginning of line
                if (command_mode) {
                    line_elems[selected_line].focus();
                    setIndex(0, line_elems[selected_line]);
                    farthest_index = getIndex();
                }
                break;

            case "o": // insert new line below and enter insert mode
                if (command_mode) {
                    command_mode = false;
                    contents.splice(selected_line + 1, 0, "");
                    if ($curr_file !== null) {
                        $curr_file.content = contents;
                    }
                    selected_line++;
                    line_elems[selected_line].focus();
                }
                break;

        }
        if (!command_mode) {
            farthest_index = getIndex();
        }
    }
</script>

<div class="container">
    {#if $curr_file !== null && $curr_file.content !== null}
    {#each contents as line_content, index}
    <div class="line-container" style="--linenum-width: {longest_lineno_len+1}rem">
        <span class="linenum"><pre>{index + 1}</pre></span>
        <pre 
            bind:this={line_elems[index]} 
            bind:innerText={$curr_file.content[index]}
            data-command-mode={command_mode}
            on:keydown={handleKeyDown} 
            on:click={handleClick}
            on:focus={handleFocus}
            contenteditable
            >{line_content}</pre>
    </div>
    {/each}
    {/if}
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
        display: grid;
        grid-template-rows: auto;
        grid-template-columns: var(--linenum-width) 1fr;
    }

    div.line-container:has(pre[contenteditable]:focus) {
        -border-top: 1px solid var(--opaque-border);
        -border-bottom: 1px solid var(--opaque-border);
        outline: 1px solid var(--opaque-border);
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
