<script lang="ts">
	import Anchor from '$lib/components/anchor.svelte';

	export let authorHref = '';
</script>

<article>
	<p class="content"><slot /></p>
	<p class="header">
		@{#if !authorHref}
			<slot name="author" />
		{:else}<Anchor href={`/user/${authorHref}`}>
				<slot name="author" />
			</Anchor>{/if}
	</p>
</article>

<style lang="scss">
	.header {
		font-weight: 900;
	}
	.content {
		white-space: pre-line;
		font-size: 1.5em;
		margin: 0;
	}

	article {
		margin: 1em;
	}
</style>
