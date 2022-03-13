<script lang="ts">
	import Anchor from '$lib/components/anchor.svelte';
	import HeartIcon from 'carbon-icons-svelte/lib/Favorite20';
	import FilledHeartIcon from 'carbon-icons-svelte/lib/FavoriteFilled20';
	import AddCommentIcon from 'carbon-icons-svelte/lib/AddComment20';

	import type { PublicPost } from '$lib/typings/api';
	import { createEventDispatcher } from 'svelte';

	import { me } from '$lib/data/me/store';

	const dispatch = createEventDispatcher<{ like: void }>();

	export let post: PublicPost;
	export let prefetch = true;

	$: authorHref = post.author.data.name;
	$: comments = post.comment_count;

	$: likes = post.likes_count;
	$: iLike = post.i_like;
</script>

<article class="m-1 my-4 dark:bg-slate-700 bg-slate-50 p-2 rounded">
	<div class="flex gap-2 items-center">
		<div class="h-10 w-10 rounded-full bg-pink-200" />
		<p class="font-bold text-center">
			@<Anchor href={`/user/${authorHref}`}>
				{post.author.data.name}
			</Anchor>
		</p>
		<div class="flex-grow" />
		<button class="flex gap-2 justify-center items-center" on:click={(e) => dispatch('like')}
			>{likes}
			{#if iLike && $me !== null}<FilledHeartIcon />{:else}<HeartIcon />{/if}
		</button>
		<Anchor {prefetch} href={`/post/${post.uuid}`}>
			<div class="flex gap-2 justify-center items-center">
				{comments}
				<AddCommentIcon />
			</div>
		</Anchor>
	</div>
	<div class="flex gap-2 align-center mt-2">
		<Anchor {prefetch} href={`/post/${post.uuid}`}>
			<p class="ml-12 whitespace-pre-line">{post.message}</p>
		</Anchor>
	</div>
</article>
