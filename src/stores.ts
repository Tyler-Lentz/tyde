import { writable } from 'svelte/store';
import type { Writable } from 'svelte/store';
import type { File } from './defs';

export const files: Writable<Array<File>> = writable([]);
export const current_file: Writable<number | null> = writable(null);