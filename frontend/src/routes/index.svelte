<script lang="ts">
	import Post from '$lib/components/post.svelte';
	import CbOnBottom from '$lib/components/cbOnBottom.svelte';
	import CenterContainer from '$lib/components/centerContainer.svelte';

	import SendIcon from 'carbon-icons-svelte/lib/SendFilled20';

	import type { PublicPost } from '$lib/typings/api';
	import { paths } from '$lib/utils/fetch';
	import { me } from '$lib/data/me/store';
	import Posts from '$lib/components/posts.svelte';
	import PostInput from '$lib/components/postInput.svelte';

	let feed: PublicPost[] = [];
	let isFetching = false;
	let done = false;

	const get_results = async () => {
		isFetching = true;

		const results = await paths.post.feed(feed.length);

		done = results.data?.length == 0;
		for (const result of results.data || []) if (result.data) feed.push(result.data);

		feed = feed;
		isFetching = false;
	};

	get_results();

	let message = '';

	const handlePost = async () => {
		const result = await paths.post.create({ message });

		if (result.data) {
			feed = [result.data, ...feed];
			message = '';
		}
	};
</script>

<CenterContainer>
	{#if $me}
		<PostInput bind:value={message} on:submit={handlePost} />
	{/if}

	<Posts bind:feed />
	{#if !done}
		<CbOnBottom
			on:intersect={async (v) => {
				if (v && !isFetching) {
					isFetching = true;
					await get_results();
				}
			}}
		/>
	{/if}
</CenterContainer>
