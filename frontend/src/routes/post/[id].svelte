<script lang="ts" context="module">
	import { paths } from '$lib/utils/fetch';
	import type { ErrorLoad } from '@sveltejs/kit';

	export const load: ErrorLoad = async ({ params }) => {
		const data = await paths.post.post(params.id);

		return {
			props: { data },
		};
	};
</script>

<script lang="ts">
	import CenterContainer from '$lib/components/centerContainer.svelte';
	import Posts from '$lib/components/posts.svelte';
	import Post from '$lib/components/post.svelte';
	import PostInput from '$lib/components/postInput.svelte';
	import Cataas from '$lib/components/cataas.svelte';

	import type { ApiResult, PostError, PublicPost } from '$lib/typings/api';
	import { me } from '$lib/data/me/store';
	import CbOnBottom from '$lib/components/cbOnBottom.svelte';

	export let data: ApiResult<PublicPost, PostError>;

	let isFetching = true;
	let done = false;

	const post = data.data;

	let feed: PublicPost[] = [];

	const get_results = async () => {
		if (post) {
			const results = await paths.post.comments(post.uuid, feed.length);

			done = results.data.length == 0;
			for (const result of results.data || []) if (result.data) feed.push(result.data);

			feed = feed;
			isFetching = false;
		}
	};

	get_results();

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
	{#if post}
		<Post {post} />

		{#if $me}
			<PostInput
				bind:value={message}
				on:submit={handlePost}
				placeholder="How does this make you feel?"
			/>
		{/if}

		<div class="m-4">
			<Posts bind:feed>
				<div class="text-center">There are no comments here,<br /> Be the first.</div>
			</Posts>
			<CbOnBottom
				on:intersect={async (v) => {
					if (v && !isFetching) {
						isFetching = true;
						await get_results();
					}
				}}
			/>
		</div>
	{:else}
		<div style="margin-top:50vh;transform:translateY(-50%)" class="rounded overflow-hidden">
			<Cataas gif say="Omg another 404" width={2000} />
		</div>
	{/if}
</CenterContainer>
