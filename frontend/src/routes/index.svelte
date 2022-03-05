<script lang="ts">
	import Encapsulator from '$lib/components/encapsulator.svelte';
	import Post from '$lib/components/post.svelte';
	import CbOnBottom from '$lib/components/cbOnBottom.svelte';

	import type { PublicPost } from '$lib/typings/api';
	import { paths } from '$lib/utils/fetch';
	import { me } from '$lib/data/me/store';
	import CenterContainer from '$lib/components/centerContainer.svelte';

	let feed: PublicPost[] = [];
	let isFetching = false;

	const get_results = async () => {
		isFetching = true;

		const results = await paths.post.feed(feed.length);
		for (const result of results.data || []) if (result.data) feed.push(result.data);

		feed = feed;
		isFetching = false;
	};

	get_results();

	let message = '';

	const handlePost = async (e: Event) => {
		e.preventDefault();

		const result = await paths.post.create({ message });

		if (result.data) {
			feed = [result.data, ...feed];
			message = '';
		}
	};
</script>

<Encapsulator>
	<CenterContainer>
		{#if $me}
			<form on:submit={handlePost}>
				<textarea name="message" cols="30" rows="10" bind:value={message} />
				<button>Post</button>
			</form>
		{/if}

		{#each feed as result}
			<Post authorHref={result.author.data.name}>
				<svelte:fragment slot="author">
					{result.author.data.name}
				</svelte:fragment>
				{result.message}
			</Post>
		{/each}
		<CbOnBottom
			on:intersect={async (v) => {
				if (v && !isFetching) {
					isFetching = true;
					await get_results();
				}
			}}
		/>
	</CenterContainer>
</Encapsulator>

<style lang="scss">
	form {
		gap: 1em;
		display: flex;
		flex-direction: column;
	}
</style>
