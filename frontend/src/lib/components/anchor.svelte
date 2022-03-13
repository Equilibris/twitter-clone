<script lang="ts">
	import { goto, prefetch as pf } from '$app/navigation';
	import { browser } from '$app/env';

	export let href: string;
	export let prefetch = true;

	if (browser && prefetch) pf(href);
</script>

<a
	{href}
	on:click={(e) => {
		e.preventDefault();
		goto(href);
	}}><slot /></a
>
