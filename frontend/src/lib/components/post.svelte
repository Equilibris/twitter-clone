<script lang="ts">
	import Anchor from '$lib/components/anchor.svelte';

	export let authorHref = '';
</script>

<article class="m-1">
	<p class="whitespace-pre-line"><slot /></p>
	<p class="font-bold">
		@{#if !authorHref}
			<slot name="author" />
		{:else}<Anchor href={`/user/${authorHref}`}>
				<slot name="author" />
			</Anchor>{/if}
	</p>
</article>
