<script lang="ts">
	import Post from './post.svelte';

	import type { PublicPost } from '$lib/typings/api';

	export let feed: PublicPost[];
</script>

{#each feed as result (result.uuid)}
	<Post pubPost={result}>
		<svelte:fragment slot="author">
			{result.author.data.name}
		</svelte:fragment>
		{result.message}
	</Post>
{:else}
	<slot>No posts were found</slot>
{/each}
