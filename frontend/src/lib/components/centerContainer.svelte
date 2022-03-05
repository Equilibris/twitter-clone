<div class="outer">
	<div class="inner">
		<slot />
	</div>
</div>

<style lang="scss">
	.outer {
		display: flex;
		justify-content: center;
		align-items: center;

		margin: 1em;

		.inner {
			width: 70ch;
			max-width: 100vw;
		}
	}
</style>
