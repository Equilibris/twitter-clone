import { writable } from 'svelte/store';
import type { Me } from '$lib/typings/api';

export const me = writable<Me | null>(null);
