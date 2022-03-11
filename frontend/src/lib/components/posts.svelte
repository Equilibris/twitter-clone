<script lang="ts">
	import Post from './post.svelte';

	import type { PublicPost } from '$lib/typings/api';
	import { paths } from '$lib/utils/fetch';

	export let feed: PublicPost[];
</script>

{#each feed as result, index (result.uuid)}
	<Post
		pubPost={result}
		on:like={async () => {
			const response = await paths.post.toggleLike(result.uuid);

			if (response.data) {
				feed[index] = response.data;
			}
		}}
	>
		<svelte:fragment slot="author">
			{result.author.data.name}
		</svelte:fragment>
		{result.message}
	</Post>
{:else}
	<slot>No posts were found</slot>
{/each}
