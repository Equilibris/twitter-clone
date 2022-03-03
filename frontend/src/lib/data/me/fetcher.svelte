<script lang="ts">
	import { me } from './store';
	import { get } from '$lib/utils/fetch';
	import type { Paths } from '$lib/typings/api';
	import { onDestroy } from 'svelte';

	const unsubscribe = me.subscribe((v) => {
		if (v && window.sessionStorage) window.sessionStorage.setItem('user', JSON.stringify(v));
	});

	const get_me = async () => {
		// Content cacheing and Hytration insurence
		// if (typeof window !== 'undefined' && window.sessionStorage) {
		// 	const v = window.sessionStorage.getItem('user');
		// 	if (v)
		// 		try {
		// 			me.set(JSON.parse(v));
		// 		} catch (_) {}
		// }

		const result = await get<Paths['user']['me']>('/user/me');

		if (result.data) me.set(result.data);
	};
	get_me();

	onDestroy(unsubscribe);
</script>
