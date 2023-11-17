<script lang="ts">
    import { appWindow } from '@tauri-apps/api/window';
    import { invoke } from '@tauri-apps/api/tauri'
    import Topbar from './Topbar.svelte';
    import TextEditor from './TextEditor.svelte';
    import EditorConsole from './EditorConsole.svelte';
    import Terminal from './Terminal.svelte';
    import { TDir, TFile } from '../file';
    import {FNode} from '../file'
    import { root, opened_files, curr_file } from '../stores';
	import WorkspaceSidebar from './WorkspaceSidebar.svelte';
	import { onDestroy } from 'svelte';

    let econsole: EditorConsole;

    appWindow.listen('open-file', (event: any) => {
        opened_files.update((ofiles) => {
            if (!("File" in event.payload)) {
                econsole.add("Error opening file");
                return ofiles;
            }
            
            let path = event.payload.File.path;
            let content = event.payload.File.content;

            let file: TFile | undefined = undefined;
            if ($root !== null) {
                file = $root.search(path);
                if (file !== undefined) {
                    file.content = content;
                }
            }

            if (file === undefined) {
                file = new TFile(path, content);
            }

            for (const f of $opened_files) {
                if (f.path === file.path) {
                    // file already opened
                    curr_file.set(file);
                    return ofiles;
                }
            }

            ofiles.push(file);
            curr_file.set(file);
            econsole.add(`Opened ${file.path}`);
            return ofiles;
        });
    });

    appWindow.listen('save-file', (_) => {
        if ($curr_file !== null) {
            if ($curr_file.path === "") {
                econsole.add(`Cannot save unnamed file`);
                return;
            }

            invoke('save_file', { fpath: $curr_file.path, content: $curr_file.content?.join("\n") }).then((path) => {
                econsole.add(`Saved ${path}`)
                if ($curr_file !== null) {
                    $curr_file.mutated = false;
                }
            });
        }
    });

    appWindow.listen('save-as-file', (_) => {
        if ($curr_file !== null) {
            invoke('save_as_file', {content: $curr_file.content?.join("\n") });
        } else {
            econsole.add("No file currently loaded.");
        }
    });

    appWindow.listen('save-as-file-completed', (event: any) => {
        if ("Ok" in event.payload && $curr_file != null) {
            let filepath = event.payload.Ok;

            if ($curr_file.name == "") {
                econsole.add(`Saved unnamed file as ${filepath}.`);
            } else {
                econsole.add(`Saved ${$curr_file.path} as ${filepath}.`)
            }

            $curr_file.path = filepath;
            $curr_file.name = filepath.split('/').at(-1) || filepath;
        } else {
            econsole.add(event.payload.Err);
        }
    });

    appWindow.listen('new-file', (_) => {
        opened_files.update((files) => {
            let file = new TFile("", "");
            files.push(file);
            curr_file.set(file);
            econsole.add(`Created new file`);
            return files;
        });
    });

    let empty_file = new TFile("null", null);

    appWindow.listen('open-directory', (msg: any) => {
        let top = new FNode(msg.payload, undefined);
        if (top.dir === undefined) {
            econsole.add("Error opening directory: no root found.");
            return;
        }
        
        root.set(top);
    });

    let old_num_files: number = 0;

    let unsub = opened_files.subscribe((ofiles) => {
        if (ofiles.length > old_num_files) {
            // New opened file at end of list
            curr_file.set(ofiles.at(-1) || null);
        } else if (ofiles.length < old_num_files) {
            if ($curr_file !== null && !ofiles.includes($curr_file)) {
                // current file was closed
                curr_file.set(null);
            }
        }

        old_num_files = ofiles.length;
    });
    onDestroy(() => {unsub();});

    let drag_container: HTMLDivElement;
    let drag_wrapper: HTMLDivElement;
    let is_dragging: boolean = false;
    let text_editor: TextEditor;
    function handleDragStart(event: MouseEvent) {
        is_dragging = true;
    };
    
    function handleDragEnd(event: MouseEvent) {
        is_dragging = false;
    }

    function handleDrag(event: MouseEvent) {
        if (is_dragging) {
            console.log("dragging")

            let container_top = drag_container.offsetTop;
            let pointer_relative_ypos = event.clientY - container_top;
            let text_editor_min_height = 60; // minimum size the editor can take up

            // drag_wrapper.style.height = (Math.max(text_editor_min_height, pointer_relative_ypos - 8)) + 'px';
            // drag_wrapper.style.flexGrow = '0';
        }
    }

</script>

<div class="top">
    <Topbar /> 
    <main class="center-area">
        <WorkspaceSidebar />
        <div class="drag-container" bind:this={drag_container}>
            <div class="drag-wrapper" bind:this={drag_wrapper}>
                <TextEditor bind:this={text_editor}/>
            </div>
            <div class="drag-bar" 
                on:mousedown={handleDragStart} 
                on:mouseup={handleDragEnd}
                on:mousemove={handleDrag}>
            </div>
            <div class="drag-wrapper">
                <Terminal />
            </div>
        </div>
    </main>
    <EditorConsole bind:this={econsole}/>
</div>

<style>
    .top {
        overflow: hidden;
    }

    main.center-area {
        height: var(--center-area-height);
    }

    div.top, div.drag-container {
        background-color: var(--darkest-bg-color);
        display: flex;
        flex-direction: column;
        justify-content: flex-start;
        align-items: flex-start;
        width: 100%;
        height: 100%;
    }

    div.drag-bar {
        height: 2px;
        border: double 2px var(--darkest-bg-color);
        width: 100%;
        background-color: var(--dark-highlight-color);
    }

    div.drag-bar:hover {
        background-color: var(--highlight-color);
        cursor: ns-resize;
    }

    div.drag-wrapper {
        flex: 1 1 auto;
        width: 100%;
        box-sizing: border-box;
    }

    main {
        display: flex;
        flex-direction: row;
        justify-content: flex-start;
        align-items: flex-start;
        width: 100%;
        height: 100%;
    }
</style>