import { writable } from 'svelte/store';
import type { Writable } from 'svelte/store';
import type { File } from './defs';

export const files: Writable<Array<File>> = writable([]);