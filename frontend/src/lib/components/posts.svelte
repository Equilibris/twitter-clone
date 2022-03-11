<script lang="ts">
	import Post from './post.svelte';

	import type { PublicPost } from '$lib/typings/api';
	import { paths } from '$lib/utils/fetch';

	export let feed: PublicPost[];
</script>

{#each feed as post, index (post.uuid)}
	<Post
		{post}
		on:like={async () => {
			const response = await paths.post.toggleLike(post.uuid);

			if (response.data) {
				feed[index] = response.data;
			}
		}}
	/>
{:else}
	<slot>No posts were found</slot>
{/each}
