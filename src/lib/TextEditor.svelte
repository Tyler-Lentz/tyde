<script lang="ts">
	import { onDestroy } from "svelte";
    import {curr_file} from "../stores"
    import {arrowLeft, arrowRight, setIndex, getIndex} from '../util';
    import EditorLine from "./EditorLine.svelte";
    import VirtualList from "./VirtualList.svelte";

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

    onDestroy(() => {
        unsub();
    });

    let command_mode: boolean = false;
    let selected_line: number = 0;
    
    let farthest_index: number = 0;

    let line_elems: Array<EditorLine> = [];

    let virlist: VirtualList;
    let start: number;
    let end: number;

    function getSelectedLine(): HTMLPreElement {
        return line_elems[selected_line - 1].getLine();
    }

    function handleClick(event: MouseEvent) {
        if (event.target !== null) {
            (event.target as HTMLPreElement)?.focus();
            selected_line = parseInt((event.target as HTMLPreElement).dataset["line-number"] || "0");
            farthest_index = getIndex();
        }
    }

    function handleFocus(event: FocusEvent) {
    }

    const SCROLL_AMT = 22;

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
                    console.log(selected_line)
                    getSelectedLine().focus();
                    setIndex(farthest_index, getSelectedLine());
                    if (selected_line >= end - 2) {
                        virlist.scrollBy(0, SCROLL_AMT);
                    }
                }
                break;

            case "k": // move cursor up if command mode
                if (!command_mode) {
                    break;
                }
            case "ArrowUp": // move cursor up
                if (selected_line > 0) {
                    selected_line--;
                    getSelectedLine().focus();
                    setIndex(farthest_index, getSelectedLine());
                    if (selected_line <= start + 2) {
                        virlist.scrollBy(0, -SCROLL_AMT);
                    }
                }
                break;

            case "a": // move cursor right and enter insert mode if command mode
                if (!command_mode) {
                    break;
                }
                command_mode = false;
                arrowRight(getSelectedLine());
                farthest_index = getIndex();
                break;

            case "l": // move cursor right if command mode
                if (!command_mode) {
                    break;
                }
            case "ArrowRight": // move cursor right
                arrowRight(getSelectedLine());
                farthest_index = getIndex();
                break;

            case "h": // move cursor left if command mode
                if (!command_mode) {
                    break;
                }
            case "ArrowLeft": // move cursor left
                arrowLeft(getSelectedLine());
                farthest_index = getIndex();
                break;

            case "i": // enter insert mode
                command_mode = false;
                break;

            case "$": // move cursor to end of line
                if (command_mode) {
                    getSelectedLine().focus();
                    setIndex(getSelectedLine().innerText.length, getSelectedLine());
                    farthest_index = getIndex();
                }
                break;
            case "0": // move cursor to beginning of line
                if (command_mode) {
                    getSelectedLine().focus();
                    setIndex(0, getSelectedLine());
                    farthest_index = getIndex();
                }
                break;

            case "Enter":
                if (command_mode) {
                    break;
                } else {
                    event.preventDefault();
                    command_mode = true; // trick into going into lower block
                }
            case "o": // insert new line below and enter insert mode
                if (command_mode) {
                    command_mode = false;
                    contents.splice(selected_line, 0, "");
                    if ($curr_file !== null) {
                        $curr_file.content = contents;
                    }
                    selected_line++;
                    getSelectedLine().focus();
                    if (selected_line >= end - 2) {
                        virlist.scrollBy(0, SCROLL_AMT);
                    }
                }
                break;

        }
        if (!command_mode) {
            farthest_index = getIndex();
        }
    }
</script>

<div class="container" >
    {#if $curr_file !== null && $curr_file.content !== null}
    <!-- {#each contents as line_content, index}
    <EditorLine 
        init_content={line_content}
        line_number={index + 1}
        command_mode={command_mode}
        handleClick={handleClick}
        handleFocus={handleFocus}
        handleKeyDown={handleKeyDown}
        bind:this={line_elems[index]}
        bind:innerText={$curr_file.content[index]}
        />
    {/each} -->
    <VirtualList bind:this={virlist} items={contents.map((e, i) => [e, i])} let:item bind:start bind:end>
        <EditorLine 
            init_content={item[0]}
            line_number={item[1] + 1}
            command_mode={command_mode}
            handleClick={handleClick}
            handleFocus={handleFocus}
            handleKeyDown={handleKeyDown}
            bind:this={line_elems[item[1]]}
            bind:innerText={$curr_file.content[item[1]]}
            />
    </VirtualList>
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
</style>
