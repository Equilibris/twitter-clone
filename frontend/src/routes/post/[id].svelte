<script lang="ts" context="module">
	import { paths } from '$lib/utils/fetch';
	import type { ErrorLoad } from '@sveltejs/kit';

	export const load: ErrorLoad = async ({ params }) => {
		const dataStream: PublicPost[] = [];

		let fetchId: string | null = params.id;

		do {
			const data = await paths.post.post(fetchId);

			if (data.data) {
				dataStream.push(data.data);
				fetchId = data.data.comment;
			} else {
				break;
			}
		} while (fetchId);

		return {
			props: {
				dataStream: dataStream.reverse(),
				serverTime: Date.now(),
				doesExist: !!dataStream.length,
			},
		};
	};
</script>

<script lang="ts">
	import CenterContainer from '$lib/components/centerContainer.svelte';
	import Posts from '$lib/components/posts.svelte';
	import PostInput from '$lib/components/postInput.svelte';
	import Cataas from '$lib/components/cataas.svelte';

	import type { PublicPost } from '$lib/typings/api';
	import { me } from '$lib/data/me/store';
	import CbOnBottom from '$lib/components/cbOnBottom.svelte';

	export let dataStream: PublicPost[];
	export let serverTime: number;
	export let doesExist: boolean;

	let isFetching = true;
	let done = false;
	let feed: PublicPost[] = [];

	let post: PublicPost = dataStream[dataStream.length - 1];
	$: serverTime, (post = dataStream[dataStream.length - 1]);

	const get_results = async () => {
		if (post) {
			const results = await paths.post.comments(post.uuid, feed.length);

			for (const result of results.data || []) if (result.data) feed.push(result.data);

			done = results.data?.length === 0;
			feed = feed;
			isFetching = false;
		}
	};

	const init = () => {
		isFetching = true;
		done = false;

		feed = [];

		get_results();
	};

	$: serverTime, init();

	let message = '';

	const handlePost = async () => {
		const result = await paths.post.createComment({ message, post: post.uuid });

		if (result.data) {
			feed = [result.data, ...feed];
			message = '';
		}
	};
</script>

<CenterContainer>
	{#if doesExist}
		<Posts disablePrefetch disableInvalidation feed={dataStream} />

		{#if $me}
			<PostInput
				bind:value={message}
				on:submit={handlePost}
				placeholder="How does this make you feel?"
			/>
		{/if}

		<div class="m-4">
			<Posts disablePrefetch bind:feed>
				<div class="text-center">There are no comments here,<br /> Be the first.</div>
			</Posts>
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
		</div>
	{:else}
		<div style="margin-top:50vh;transform:translateY(-50%)" class="rounded overflow-hidden">
			<Cataas gif say="Omg another 404" width={2000} />
		</div>
	{/if}
</CenterContainer>
