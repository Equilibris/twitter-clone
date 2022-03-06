<script lang="ts">
	import '../app.css';
	import Fetcher from '$lib/data/me/fetcher.svelte';
	import { me } from '$lib/data/me/store';
	import Anchor from '$lib/components/anchor.svelte';

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
	<header class="flex justify-between items-center w-full flex-row">
		<Anchor href="/">
			<b>Shwitter</b>
		</Anchor>
		<form action="/search">
			<input type="text" name="query" />
			<button>Search</button>
		</form>
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
