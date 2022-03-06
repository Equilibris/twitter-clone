<script lang="ts">
	import Anchor from '$lib/components/anchor.svelte';
	import HeartIcon from 'carbon-icons-svelte/lib/Favorite20';

	export let authorHref = '';
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
		<div class="w-10 h-10 flex justify-center items-center flex-col"><HeartIcon />0</div>
		<p class="whitespace-pre-line"><slot /></p>
	</div>
</article>
