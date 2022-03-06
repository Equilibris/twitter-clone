<script lang="ts">
	import { createEventDispatcher, onDestroy, onMount } from 'svelte';

	const dispatch = createEventDispatcher<{ intersect: boolean }>();

	let ioContainer: HTMLDivElement;

	let isIntersecting = false;

	const eventListenr = setInterval(() => {
		if (isIntersecting) dispatch('intersect', true);
	}, 1000);

	onDestroy(() => {
		clearInterval(eventListenr);
	});

	onMount(() => {
		if (typeof IntersectionObserver !== 'undefined') {
			const io = new IntersectionObserver(
				async (v) => {
					isIntersecting = v[0].isIntersecting;

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
