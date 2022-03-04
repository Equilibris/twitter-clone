<script lang="ts">
	import Encapsulator from '$lib/components/encapsulator.svelte';
	import Post from '$lib/components/post.svelte';

	import type { Paths, PublicPost } from '$lib/typings/api';
	import { get, post } from '$lib/utils/fetch';
	import { me } from '$lib/data/me/store';
	import { onMount } from 'svelte';

	let feed: PublicPost[] = [];
	let isFetching = true;

	const get_results = async () => {
		const results = await get<Paths['post']['feed']>(`/post/feed/${feed.length}`);

		for (const result of results.data || []) if (result.data) feed.push(result.data);

		feed = feed;
		isFetching = false;
	};

	get_results();

	let message = '';

	const handlePost = async (e: Event) => {
		e.preventDefault();

		const result = await post<Paths['post']['create']>('/post/create', { message });

		if (result.data) {
			feed = [result.data, ...feed];
			message = '';
		}
	};

	let ioContainer: HTMLDivElement;

	onMount(() => {
		if (typeof IntersectionObserver !== 'undefined') {
			const io = new IntersectionObserver(
				async (v) => {
					if (v[0].isIntersecting && !isFetching) {
						isFetching = true;
						await get_results();
					}
				},
				{
					rootMargin: '1500px',
					threshold: 0
				}
			);

			io.observe(ioContainer);

			return () => io.unobserve(ioContainer);
		}
	});
</script>

<Encapsulator>
	<div class="container">
		<div>
			{#if $me}
				<form on:submit={handlePost}>
					<textarea name="message" cols="30" rows="10" bind:value={message} />
					<button>Post</button>
				</form>
			{/if}

			{#each feed as result}
				<Post>
					<svelte:fragment slot="author">
						{result.author.data.name}
					</svelte:fragment>
					{result.message}
				</Post>
			{/each}
			<div id="intersector" bind:this={ioContainer} />
		</div>
	</div>
</Encapsulator>

<style lang="scss">
	form {
		gap: 1em;
		display: flex;
		flex-direction: column;
	}
	.container {
		display: flex;
		justify-content: center;
		align-items: center;
	}
</style>
