<script>
    import { appWindow } from '@tauri-apps/api/window';

    let output = '';
    let num_lines = 0;
    let line_nums = '';

    let font_size = 12; //pt

    $: num_lines = output.split('\n').length;
    $: line_nums = Array(num_lines).fill(0).map((_, num) => `${num}`).join('\n');
    $: max_line_size = String(num_lines - 1).length * font_size;

    appWindow.listen('open-file', (event) => {
        output = event.payload.file;
    });
</script>

<div >
    <textarea style:font_size class="line_nums" readonly bind:value={line_nums} style:width="{max_line_size}px" ></textarea>
    <textarea style:font_size class="editor" bind:value={output} ></textarea>
</div>

<style>
    div {
        display: flex;
        flex-direction: row;
        width: 100%;
    }

    textarea {
        resize: none;
        box-shadow: none;
        outline: 0;
        border: none;
        border-left: 1px solid var(--dark-highlight-color);
        font-family: monospace;
        background-color: var(--dark-bg-color);
    }

    textarea.line_nums {
        height: 95vh;
        color: var(--text-highlight-color);
        overflow: hidden;
    }

    textarea.line_nums::selection {
        background-color: none;
    }

    textarea.editor {
        width: 95%;
        height: var(--height);
        color: var(--text-default-color);
    }

    textarea.editor::selection {
        background-color: var(--text-highlight-color);
        color: var(--text-highlight-color);
    }

</style>
