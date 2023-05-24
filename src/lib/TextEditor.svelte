<script lang="ts">
	import { onDestroy } from "svelte";
    import {curr_file} from "../stores"
    import {arrowDown, arrowUp, arrowLeft, arrowRight, insertAtCursor} from '../util';

    let vim_mode: boolean;

    let contents: Array<[string, string]> = [];

    let unsub = curr_file.subscribe((new_file) => {
        if (new_file !== null && new_file.content !== null) {
            contents = new_file.content
                        .split('\n')
                        .map((line_content, line_number) => [formatLineNumber(line_number + 1), line_content])
        } else {
            contents = [];
        }
    });

    function formatLineNumber(line_number: number):string {
        let str_len = String(contents.length).length; // how many chars the largest line number is
        return String(line_number).padEnd(str_len, ' ');
    }

    onDestroy(() => {
        unsub();
    });
</script>

<div class="container">
    {#each contents as [line_number, line_content]}
    <div>
        <span class="linenum">{line_number}</span><pre contenteditable>{line_content}</pre>
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

    span.linenum {
        display: inline;
        padding:0;
        margin:0;
        margin-right:1rem;
        user-select: none;
        -webkit-user-select: none;
        color: var(--text-highlight-color);
        font-family: monospace;
    }

    pre[contenteditable] {
        display: inline;
        margin: 0;
        padding: 0;
        outline: 0;
        color: var(--text-default-color);
        font: monospace;
        width: 100%;
    }

</style>
