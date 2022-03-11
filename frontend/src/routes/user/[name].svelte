<script lang="ts" context="module">
	import { paths } from '$lib/utils/fetch';
	import type { ErrorLoad } from '@sveltejs/kit';
	import type { ApiResult, GetUserError, PublicPost, PublicUser } from '$lib/typings/api';

	export const load: ErrorLoad = async ({ params }) => {
		const data = await paths.user.getByName(params.name);

		return {
			props: { data },
		};
	};
</script>

<script lang="ts">
	import CenterContainer from '$lib/components/centerContainer.svelte';
	import CbOnBottom from '$lib/components/cbOnBottom.svelte';
	import Posts from '$lib/components/posts.svelte';

	export let data: ApiResult<PublicUser, GetUserError>;

	let isFetching = true;
	let done = false;

	const user = data.data;

	let feed: PublicPost[] = [];

	const get_results = async () => {
		if (user) {
			const results = await paths.post.authorFeed(user.uuid, feed.length);

			done = results.data.length == 0;
			for (const result of results.data || []) if (result.data) feed.push(result.data);

			feed = feed;
			isFetching = false;
		}
	};

	get_results();
</script>

<CenterContainer>
	{#if user}
		<div class="flex items-center gap-4">
			<div class="w-20 h-20 rounded-full bg-pink-400" />
			<h1 class="font-bold text-4xl">@{user.name}</h1>
		</div>

		<Posts bind:feed>User has not posted yet</Posts>
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
	{:else}
		<h1>Oh shit an error occured (404)</h1>
	{/if}
</CenterContainer>
