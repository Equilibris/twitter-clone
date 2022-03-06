<script lang="ts">
	import CbOnBottom from '$lib/components/cbOnBottom.svelte';
	import CenterContainer from '$lib/components/centerContainer.svelte';

	import { page } from '$app/stores';
	import type { PublicPost } from '$lib/typings/api';
	import { paths } from '$lib/utils/fetch';
	import Post from '$lib/components/post.svelte';
	import Cataas from '$lib/components/cataas.svelte';

	const query = $page.url.searchParams.get('query');

	let isFetching = true;
	let done = false;

	let feed: PublicPost[] = [];

	const get_results = async () => {
		if (query) {
			const results = await paths.post.search(query, feed.length);

			done = results.data.length == 0;
			for (const result of results.data || []) if (result.data) feed.push(result.data);

			feed = feed;
			isFetching = false;
		}
	};

	get_results();
</script>

<CenterContainer>
	{#each feed as result}
		<Post authorHref={result.author.data.name}>
			<svelte:fragment slot="author">
				{result.author.data.name}
			</svelte:fragment>
			{result.message}
		</Post>
	{:else}
		<div style="margin-top:50vh;transform:translateY(-50%)" class="rounded overflow-hidden">
			<Cataas gif say="Lol no results" width={2000} />
		</div>
	{/each}
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
