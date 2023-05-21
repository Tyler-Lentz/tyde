import { writable } from 'svelte/store';
import type { Writable } from 'svelte/store';
import type { FNode, TFile } from './file'

export const root: Writable<FNode|null> = writable(null);
export const opened_files: Writable<Array<TFile>> = writable([]);
export const curr_file: Writable<TFile|null> = writable(null);

export const econsole: Writable<string> = writable("");