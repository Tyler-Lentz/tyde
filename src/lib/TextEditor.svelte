<script>
    import { appWindow } from '@tauri-apps/api/window';

    let output = '';

    appWindow.listen('open-file', (event) => {
        output = event.payload.file;
    });
</script>

<textarea bind:value={output}></textarea>

<style>
    textarea {
        width: 100%;
        height: 95vh;
        font-family: monospace;
        font-size: 12pt;
        resize: none;
        box-shadow: none;
        outline: 0;
        border: none;
        border-left: 1px solid var(--dark-highlight-color);
        background-color: var(--dark-bg-color);
        color: var(--text-default-color);
    }

    textarea::selection {

        background-color: var(--highlight-color);
        color: var(--highlight-color);
    }

</style>
