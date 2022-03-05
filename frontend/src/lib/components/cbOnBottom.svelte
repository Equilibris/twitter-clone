<script lang="ts">
	import { createEventDispatcher, onMount } from 'svelte';

	const dispatch = createEventDispatcher<{ intersect: boolean }>();

	let ioContainer: HTMLDivElement;

	onMount(() => {
		if (typeof IntersectionObserver !== 'undefined') {
			const io = new IntersectionObserver(
				async (v) => {
					dispatch('intersect', v[0].isIntersecting);
				},
				{
					rootMargin: '1000px',
					threshold: 0,
				},
			);

			io.observe(ioContainer);

			return () => io.unobserve(ioContainer);
		}
	});
</script>

<div bind:this={ioContainer} />
