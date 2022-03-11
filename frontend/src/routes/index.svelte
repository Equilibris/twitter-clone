<script lang="ts">
	import Post from '$lib/components/post.svelte';
	import CbOnBottom from '$lib/components/cbOnBottom.svelte';
	import CenterContainer from '$lib/components/centerContainer.svelte';

	import SendIcon from 'carbon-icons-svelte/lib/SendFilled20';

	import type { PublicPost } from '$lib/typings/api';
	import { paths } from '$lib/utils/fetch';
	import { me } from '$lib/data/me/store';
	import Posts from '$lib/components/posts.svelte';

	let feed: PublicPost[] = [];
	let isFetching = false;
	let done = false;

	const get_results = async () => {
		isFetching = true;

		const results = await paths.post.feed(feed.length);

		done = results.data.length == 0;
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

<CenterContainer>
	{#if $me}
		<form on:submit={handlePost} class="gap-1 flex flex-col bg-pink-50 text-black rounded p-2 m-1">
			<textarea
				placeholder="What's on your mind?"
				class="bg-transparent focus:outline-none resize-none"
				name="message"
				cols="20"
				rows="7"
				bind:value={message}
				on:keypress={(e) => {
					if (e.shiftKey && e.keyCode === 13) handlePost(e);
				}}
			/>
			<button class="ml-auto bg-pink-400 h-10 w-10 flex items-center justify-center rounded-full"
				><SendIcon class="fill-white transition hover:fill-pink-200" /></button
			>
		</form>
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
