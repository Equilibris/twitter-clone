<article>
	<p class="content"><slot /></p>
	<p class="header">@<slot name="author">[[FAILED TO FETCH AUTHOR]]</slot></p>
</article>

<style lang="scss">
	.header {
		font-weight: 900;
	}
	.content {
		font-size: 1.5em;
		margin: 0;
	}

	article {
		margin: 1em;
	}
</style>
