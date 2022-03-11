<script lang="ts">
	import Anchor from '$lib/components/anchor.svelte';
	import HeartIcon from 'carbon-icons-svelte/lib/Favorite20';
	import FilledHeartIcon from 'carbon-icons-svelte/lib/FavoriteFilled20';

	import type { PublicPost } from '$lib/typings/api';
	import { createEventDispatcher } from 'svelte';

	import { me } from '$lib/data/me/store';

	const dispatch = createEventDispatcher<{ like: void }>();

	export let pubPost: PublicPost;

	$: authorHref = pubPost.author.data.name;
	$: likes = pubPost.likes_count;
	$: iLike = pubPost.i_like;
</script>

<article class="m-1 my-4 dark:bg-slate-700 bg-slate-50 p-2 rounded">
	<div class="flex gap-2 items-center">
		<div class="h-10 w-10 rounded-full bg-pink-200" />
		<p class="font-bold text-center">
			@{#if !authorHref}
				<slot name="author" />
			{:else}
				<Anchor href={`/user/${authorHref}`}>
					<slot name="author" />
				</Anchor>
			{/if}
		</p>
	</div>
	<div class="flex gap-2 align-center mt-2">
		<div
			on:click={() => dispatch('like')}
			class="w-10 h-10 flex justify-center items-center flex-col select-none"
		>
			{#if iLike && $me !== null}<FilledHeartIcon />{:else}<HeartIcon />{/if}{likes}
		</div>
		<p class="whitespace-pre-line"><slot /></p>
	</div>
</article>
