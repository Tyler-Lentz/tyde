import { writable } from 'svelte/store';
import type { Writable } from 'svelte/store';
import type { TFile } from './file'

export const files: Writable<Array<TFile>> = writable([]);

export const econsole: Writable<string> = writable("");