<script lang="ts">
    import { appWindow } from '@tauri-apps/api/window';
    import { invoke } from '@tauri-apps/api/tauri'
    import Topbar from './Topbar.svelte';
    import TextEditor from './TextEditor.svelte';
    import EditorConsole from './EditorConsole.svelte';
    import { Directory, File } from '../defs';
    import { files } from '../stores';
	import WorkspaceSidebar from './WorkspaceSidebar.svelte';

    let econsole: EditorConsole;

    let current_file: number | null = null;

    appWindow.listen('open-file', (event: any) => {
        files.update((files) => {
            let file = new File(event.payload.name, event.payload.content);

            for (const f of files) {
                if (f.absPath === file.absPath) {
                    econsole.add(`File ${file.absPath} already open`);
                    return files;
                }
            }

            files.push(file);
            current_file = files.length - 1;
            econsole.add(`Opened ${event.payload.name}`);
            return files;
        });
    });

    appWindow.listen('save-file', (_) => {
        if (current_file !== null) {
            let file = $files[current_file];
            if (file.absPath === "") {
                econsole.add(`Cannot save unnamed file`);
                return;
            }

            invoke('save_file', { fpath: file.absPath, content: file.content }).then((_) => {
                econsole.add(`Saved ${file.absPath}`)
            });
        }
    });

    appWindow.listen('new-file', (_) => {
        files.update((files) => {
            let file = new File("", "");
            files.push(file);
            current_file = files.length - 1;
            econsole.add(`Created new file`);
            return files;
        });
    });


    let directory: Directory | null = null;

    let empty_file = new File("null", "");

    appWindow.listen('open-directory', (_) => {
        let dir = new Directory("/user", [
            new File("/user/test", "content"),
            new File("/user/test2", "content"),
            new File("/user/test3", "content"),
            new Directory("/user/tuas", [
                new Directory("/user/tuas/gcs", [
                    new File("/user/tuas/gcs/main.go", "content"),
                    new File("/user/tuas/gcs/datatypes.go", "content"),
                    new Directory("/user/tuas/gcs/internal", [
                        new File("/user/tuas/gcs/internal/server.go", "content")
                    ]),
                    new File("/user/tuas/gcs/amogus.go", "content"),
                ]),
                new File("/user/tuas/temp.txt", "content"),
                new Directory("/user/tuas/obc", [
                    new File("/user/tuas/obc/main.py", "content"),
                    new File("/user/tuas/obc/datatypes.py", "content"),
                    new Directory("/user/tuas/obc/internal", [
                        new File("/user/tuas/obc/internal/server.py", "content")
                    ]),
                    new File("/user/tuas/obc/amogus.py", "content"),
                ])
            ])
        ]);
        directory = dir;
    });
    
</script>

<div>
    <Topbar bind:current_file/> 
    <main>
        <WorkspaceSidebar bind:directory></WorkspaceSidebar> 
        {#if current_file !== null}
            <TextEditor file={$files[current_file]} editable={true} />
        {:else}
            <TextEditor file={empty_file} editable={false}/>
        {/if}
    </main>
    <EditorConsole bind:this={econsole}/>
</div>

<style>
    div {
        background-color: var(--darkest-bg-color);
        display: flex;
        flex-direction: column;
        justify-content: flex-start;
        align-items: flex-start;
        width: 100%;
        height: 100%;
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