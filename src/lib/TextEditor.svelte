<script lang="ts">
    import type { File } from '../defs';
    import { slide } from 'svelte/transition';

    export let file: File;

    let num_lines: number = 0;
    let line_nums: String = '';

    let font_size_num: number = 16; //pt
    let font_size = `${font_size_num}px`;

    let editor_elem: HTMLTextAreaElement;
    let line_nums_elem: HTMLTextAreaElement;

    function parseScroll() {
        line_nums_elem.scrollTop = editor_elem.scrollTop; 
    }

    $: num_lines = file.content.split('\n').length;
    $: line_nums = Array(num_lines).fill(0).map((_, num) => `${num+1}`).join('\n');
    $: max_line_size = String(num_lines).length * font_size_num;

</script>

<div transition:slide>
    <textarea bind:this={line_nums_elem} class="line_nums" readonly bind:value={line_nums} style:width="{max_line_size}px" ></textarea>
    <textarea bind:this={editor_elem} on:scroll={parseScroll} class="editor" bind:value={file.content} ></textarea>
</div>

<style >
    div {
        display: flex;
        flex-direction: row;
        width: 100%;
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
        height: 90vh;
        color: var(--text-highlight-color);
        overflow: hidden;
        text-align: right;
    }

    textarea.line_nums::selection {
        background-color: none;
    }

    textarea.editor {
        width: 100%;
        border-left: 1px solid var(--dark-highlight-color);
        height: var(--height);
        color: var(--text-default-color);
    }

    textarea.editor::selection {
        background-color: var(--text-highlight-color);
        color: var(--text-highlight-color);
    }

</style>
