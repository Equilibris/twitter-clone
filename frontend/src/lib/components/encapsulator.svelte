<script lang="ts">
	import Fetcher from '$lib/data/me/fetcher.svelte';
	import { me } from '$lib/data/me/store';
	import Anchor from './anchor.svelte';

	const signOut = async (e: Event) => {
		e.preventDefault();

		if (window.sessionStorage && window.localStorage) {
			window.sessionStorage.removeItem('user');
			window.localStorage.removeItem('refresh_token');
		}

		$me = null;
	};
</script>

<main>
	<header>
		<Anchor href="/">
			<b>Shwitter</b>
		</Anchor>
		{#if $me}
			<a on:click={signOut} href="/">Sign out</a>
		{:else}
			<div>
				<Anchor href="/sign_in">Sign in</Anchor> |
				<Anchor href="/sign_up">Sign up</Anchor>
			</div>
		{/if}
	</header>
	<Fetcher />
	<slot />
</main>

<style lang="scss">
	header {
		display: flex;

		width: 100%;

		justify-content: space-evenly;
		align-items: center;
	}
</style>
