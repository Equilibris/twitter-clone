<script lang="ts">
	import Fetcher from '$lib/data/me/fetcher.svelte';
	import { me } from '$lib/data/me/store';
	import type { Paths } from '$lib/typings/api';
	import { post } from '$lib/utils/fetch';

	const signOut = async (e: Event) => {
		e.preventDefault();
		const result = await post<Paths['user']['sign_out']>('/user/sign_out', null);

		if (window.sessionStorage) {
			window.sessionStorage.removeItem('user');
		}

		$me = null;
	};
</script>

<main>
	<header>
		<b>Shwitter</b>
		{#if $me}
			<a on:click={signOut} href="/">Sign out</a>
		{:else}
			<div>
				<a href="/sign_in">Sign in</a> |
				<a href="/sign_up">Sign up</a>
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
