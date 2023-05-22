<script lang="ts">
    import { invoke } from '@tauri-apps/api/tauri';
    import { appWindow } from '@tauri-apps/api/window';
    import { Terminal } from 'xterm';
    import { FitAddon } from 'xterm-addon-fit';
    import "xterm/css/xterm.css";
    import type { Event } from '@tauri-apps/api/event';
	import { onMount } from 'svelte';

    let terminal_element: HTMLElement;
    let terminal: Terminal;
    let fit: FitAddon;

    function fitTerminal() {
        fit.fit();
        invoke("async_resize_pty", {
            rows: terminal.rows, cols: terminal.cols
        });
    }

    function writeToTerminal(ev: Event<string>) {
        terminal.write(ev.payload);
    }

    function writeToPty(data: string) {
        invoke("async_write_to_pty", {data});
    }

    onMount(async () => {
        fit = new FitAddon();
        terminal = new Terminal({
            fontFamily: "monospace",
            theme: {
                background: "rgb(30,30,30)", // --darkest-bg-color
            }
        });
        terminal.loadAddon(fit);
        terminal.open(terminal_element);

        appWindow.listen("terminal-data", writeToTerminal);
        terminal.onData(writeToPty);
        fitTerminal();

        setInterval(fitTerminal, 10); // TODO: figure out better way to do this
    })
</script>

<div bind:this={terminal_element}>

</div>

<style>
    div {
        width: 100%;
        height: var(--terminal-height);
        border: 1px solid var(--dark-highlight-color);
        box-sizing: border-box;
        border-left: none;
        border-bottom: none;
    }
</style>