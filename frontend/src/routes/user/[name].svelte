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
	import Encapsulator from '$lib/components/encapsulator.svelte';
	import CbOnBottom from '$lib/components/cbOnBottom.svelte';
	import Post from '$lib/components/post.svelte';

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

<Encapsulator>
	<CenterContainer>
		{#if user}
			<h1>@{user.name}</h1>

			{#each feed as result}
				<Post>
					<svelte:fragment slot="author">
						{result.author.data.name}
					</svelte:fragment>
					{result.message}
				</Post>
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
		{:else}
			<h1>Oh shit an error occured (404)</h1>
		{/if}
	</CenterContainer>
</Encapsulator>
