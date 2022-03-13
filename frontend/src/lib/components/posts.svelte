<script lang="ts">
	import Post from './post.svelte';

	import type { PublicPost } from '$lib/typings/api';
	import { paths } from '$lib/utils/fetch';
	import { invalidate } from '$app/navigation';

	export let feed: PublicPost[];

	export let disableInvalidation = false;
	export let disablePrefetch = false;
</script>

{#each feed as post, index (post.uuid)}
	<Post
		prefetch={!disablePrefetch}
		{post}
		on:like={async () => {
			const response = await paths.post.toggleLike(post.uuid);

			if (response.data) {
				if (!disableInvalidation) {
					await invalidate(`/user/${response.data.author.data.name}`);
					await invalidate(`/post/${response.data.uuid}`);
				}

				feed[index] = response.data;
			}
		}}
	/>
{:else}
	<slot>No posts were found</slot>
{/each}
