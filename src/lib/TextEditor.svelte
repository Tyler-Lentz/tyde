<script lang="ts">
	import { onDestroy } from "svelte";
    import {curr_file} from "../stores"
    import {arrowLeft, arrowRight, setIndex, getIndex} from '../util';
    import EditorLine from "./EditorLine.svelte";
    import VirtualList from 'svelte-tiny-virtual-list';


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
        return line_elems[selected_line].getLine();
    }
    function getSelectedContents(): string {
        return contents[selected_line];
    }
    function appendToSelectedContents(s: string) {
        contents[selected_line] += s;
    }
    function removeFromSelectedContentsAfter(index: number) {
        contents[selected_line] = contents[selected_line].slice(0, index)
    }
    function removeSelectedLine() {
        contents.splice(selected_line, 1);
    }
    function addNewlineBelow() {
        contents.splice(selected_line+1, 0, "");
    }

    // Set farthest index to current cursor position
    // relevant when using arrow keys / vim keys to move up/down
    // so it knows where to put the cursor
    function updateFarthestIndex() {
        farthest_index = getIndex();
    }

    // Set cursor to be at farthest index allowed currently
    function moveToFarthestIndex() {
        setIndex(farthest_index, getSelectedLine());
    }

    const SCROLL_AMT = 22;
    function checkForScrollDown() {
        if (selected_line >= end - 2) {
            // virlist.scrollBy(0, SCROLL_AMT);
        }
    }
    function checkForScrollUp() {
        if (selected_line <= start + 2) {
            // virlist.scrollBy(0, -SCROLL_AMT);
        }
    }

    // need to call this whenever modifying contents
    // triggers a rerender and regenerates line_num
    function cacheContents() {
        if ($curr_file !== null) {
            $curr_file.content = contents;
        }
    }

    function handleClick(event: MouseEvent) {
        if (event.target !== null) {
            (event.target as HTMLPreElement)?.focus();
            selected_line = parseInt((event.target as HTMLPreElement).dataset["line-number"] || "0") - 1;
            updateFarthestIndex();
        }
    }

    function handleFocus(event: FocusEvent) {
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
                    getSelectedLine().focus();
                    moveToFarthestIndex();
                    checkForScrollDown();
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
                    moveToFarthestIndex();
                    checkForScrollUp();
                }
                break;

            case "a": // move cursor right and enter insert mode if command mode
                if (!command_mode) {
                    break;
                }
                command_mode = false;
                arrowRight(getSelectedLine());
                updateFarthestIndex();
                break;

            case "l": // move cursor right if command mode
                if (!command_mode) {
                    break;
                }
            case "ArrowRight": // move cursor right
                arrowRight(getSelectedLine());
                updateFarthestIndex();
                break;

            case "h": // move cursor left if command mode
                if (!command_mode) {
                    break;
                }
            case "ArrowLeft": // move cursor left
                arrowLeft(getSelectedLine());
                updateFarthestIndex();
                break;

            case "i": // enter insert mode
                command_mode = false;
                break;

            case "$": // move cursor to end of line
                if (command_mode) {
                    getSelectedLine().focus();
                    setIndex(getSelectedLine().innerText.length, getSelectedLine());
                    updateFarthestIndex();
                }
                break;
            case "0": // move cursor to beginning of line
                if (command_mode) {
                    getSelectedLine().focus();
                    setIndex(0, getSelectedLine());
                    updateFarthestIndex();
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
                    let startIndex = getIndex();
                    let words_after_cursor = getSelectedContents().slice(startIndex);
                    addNewlineBelow();
                    if (event.key === "Enter") {
                        removeFromSelectedContentsAfter(startIndex);
                    }
                    selected_line++;
                    if (event.key === "Enter") {
                        appendToSelectedContents(words_after_cursor);
                    }
                    cacheContents();
                    getSelectedLine().focus();
                    checkForScrollDown();
                }
                break;

            case "Backspace": // need to check if at beginning of a line
                if (command_mode) {
                    // TODO: handle backspace on command entry
                    break;
                }
                if (getIndex() === 0) {
                    // We are at the start of the line, so we should get all of the text on the current line
                    // and move it to after the previous line but without the line break
                    let carry_over_text = getSelectedContents();
                    if (selected_line > 0) {
                        event.preventDefault();
                        removeSelectedLine();
                        selected_line--;
                        let original_text = getSelectedContents();
                        appendToSelectedContents(carry_over_text);
                        cacheContents();
                        getSelectedLine().focus();
                        setTimeout(() => {
                            // send to event loop to make sure everything has been updated by the time this is run
                            setIndex(original_text.length, getSelectedLine());
                        }, 0);
                        checkForScrollUp();
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
    <VirtualList
        width="100%"
        height={600}
        bind:itemCount={contents.length}
        itemSize={20}>
        >
        <div slot="item" let:index let:style {style}>
            <EditorLine 
                init_content={contents[index]}
                line_number={index + 1}
                command_mode={command_mode}
                handleClick={handleClick}
                handleFocus={handleFocus}
                handleKeyDown={handleKeyDown}
                bind:longest_lineno_len
                bind:this={line_elems[index]}
                bind:innerText={$curr_file.content[index]}
                />
        </div>

    </VirtualList>
    <!-- <VirtualList bind:this={virlist} items={contents.map((e, i) => [e, i])} let:item bind:start bind:end>
        <EditorLine 
            init_content={item[0]}
            line_number={item[1] + 1}
            command_mode={command_mode}
            handleClick={handleClick}
            handleFocus={handleFocus}
            handleKeyDown={handleKeyDown}
            bind:longest_lineno_len
            bind:this={line_elems[item[1]]}
            bind:innerText={$curr_file.content[item[1]]}
            />
    </VirtualList> -->
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
