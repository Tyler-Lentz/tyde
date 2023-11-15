# tyde 
<ins>**ty**</ins>ler's <ins>**d**</ins>evelopment <ins>**e**</ins>nvironment

A custom IDE developed mainly for fun and to learn Rust and Svelte through Tauri. I'm basically looking at VSCode and trying to implement whatever features I can, plus some others that I think are interesting.

## Current Functionality
- open/save files
- open directory and view all files in directory
- detect when edited file is unsaved
- console that logs file operations
- basic subset of vim commands

## TODO

### Short Term
- directory view updates in real time if file system changes
- save as option
- in-app terminal

### Long Term
- navigation shortcuts
- more vim commands
- configurable themes
- syntax highlighting
- code autocomplete
- dedicated subway surfers tab (high priority)

## How to set up

I have only done this on Ubuntu so far.

1. Install [Rust](https://www.rust-lang.org/tools/install)

2. Clone this repo

3. `npm install`

4. `cd src-tauri && cargo build`

If you run into any missing system packages, you will have to install them. 

5. `cd .. && npm run tauri dev`

Note: this may not work from inside of a VSCode terminal, for whatever reason.