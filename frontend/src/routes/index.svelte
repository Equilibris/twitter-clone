<script lang="ts">
	import Encapsulator from '$lib/components/encapsulator.svelte';
	import Post from '$lib/components/post.svelte';

	import type { Paths, PublicPost } from '$lib/typings/api';
	import { get, post } from '$lib/utils/fetch';
	import { me } from '$lib/data/me/store';

	let feed: PublicPost[] = [];

	const get_results = async () => {
		const results = await get<Paths['post']['feed']>(`/post/feed/${feed.length}`);

		for (const result of results.data || []) if (result.data) feed.push(result.data);

		feed = feed;

		console.log(feed);
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
</script>

<Encapsulator>
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
</Encapsulator>
