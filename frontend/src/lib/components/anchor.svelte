<script lang="ts">
	import { goto, prefetch } from '$app/navigation';
	import { browser } from '$app/env';

	export let href: string;

	if (browser) prefetch(href);
</script>

<a
	{href}
	on:click={(e) => {
		e.preventDefault();
		goto(href);
	}}><slot /></a
>

<style lang="scss">
	a {
		text-decoration: underline;
		text-decoration-color: #0000;

		transition: all 0.3s;
		&:hover {
			text-decoration-color: inherit;
		}
	}
</style>
