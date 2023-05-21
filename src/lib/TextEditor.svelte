<script lang="ts">
    import type {TFile} from "../file"

    export let current_file: TFile;

    let num_lines: number = 0;
    let line_nums: String = '';

    let editor_elem: HTMLTextAreaElement;
    let line_nums_elem: HTMLTextAreaElement;

    function parseScroll() {
        line_nums_elem.scrollTop = editor_elem.scrollTop; 
    }

    $: fcontent = current_file.content !== null ? current_file.content : "";
    $: num_lines = fcontent.split('\n').length;
    $: line_nums = Array(num_lines).fill(0).map((_, num) => {
        if (current_file.content !== null) {
            return `${num+1}`;
        } else {
            return '';
        }
    }).join('\n');
    $: max_line_size = String(num_lines).length * 16;

</script>

<div >
    <textarea bind:this={line_nums_elem} class="line_nums" readonly bind:value={line_nums} style:width="{max_line_size}px" ></textarea>
    <textarea bind:this={editor_elem} on:scroll={parseScroll} class="editor" bind:value={current_file.content} readonly={current_file.content === null}></textarea>
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
        min-width: 32px;
        -webkit-user-select: none;
        user-select: none;
    }

    textarea.line_nums::selection {
        background-color: none;
    }

    textarea.editor {
        height: 90vh;
        width: 100%;
        border-left: 1px solid var(--dark-highlight-color);
        color: var(--text-default-color);
    }

    textarea.editor::selection {
        background-color: var(--text-highlight-color);
        color: var(--text-highlight-color);
    }

</style>
