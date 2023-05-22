<script lang="ts">
	import { onDestroy } from "svelte";
    import type {TFile} from "../file"
    import {curr_file} from "../stores"

    let num_lines: number = 0;
    let line_nums: String = '';

    let editor_elem: HTMLTextAreaElement;
    let line_nums_elem: HTMLTextAreaElement;

    function parseScroll() {
        line_nums_elem.scrollTop = editor_elem.scrollTop; 
    }

    function setMutated() {
        if ($curr_file !== null) {
            $curr_file.mutated = true;
            curr_file.update(a => a);
        }
    }

    $: fcontent = $curr_file !== null && $curr_file.content !== null ? $curr_file.content : "";
    $: num_lines = fcontent.split('\n').length;
    $: line_nums = Array(num_lines).fill(0).map((_, num) => {
        if ($curr_file!== null && $curr_file.content !== null) {
            return `${num+1}`;
        } else {
            return '';
        }
    }).join('\n');
    $: max_line_size = String(num_lines).length * 16;

</script>

<div >
    <textarea bind:this={line_nums_elem} class="line_nums" readonly bind:value={line_nums} style:width="{max_line_size}px" ></textarea>
    {#if $curr_file !== null}
        <textarea bind:this={editor_elem} on:change={setMutated} on:scroll={parseScroll} class="editor" bind:value={$curr_file.content} readonly={$curr_file.content === null}></textarea>
    {:else}
        <textarea bind:this={editor_elem} class="editor" value="" readonly></textarea>
    {/if}
</div>

<style >
    div {
        display: flex;
        flex-direction: row;
        width: 100%;
        height: 100%;
        margin: 0;
    }

    textarea {
        resize: none;
        box-shadow: none;
        outline: 0;
        border: none;
        font-family: monospace;
        background-color: var(--dark-bg-color);
        font-size: var(--font-size);
        tab-size: 4;
    }

    textarea.line_nums {
        height: 75vh;
        color: var(--text-highlight-color);
        overflow: hidden;
        text-align: right;
        min-width: 32px;
        -webkit-user-select: none;
        user-select: none;
    }

    textarea.line_nums::selection {
        background-color: none;
    }

    textarea.editor {
        height: 75vh;
        width: 100%;
        border-left: 1px solid var(--dark-highlight-color);
        border-right: 1px solid var(--dark-highlight-color);
        color: var(--text-default-color);
    }

    textarea.editor::selection {
        background-color: var(--text-highlight-color);
        color: var(--text-highlight-color);
    }

</style>
