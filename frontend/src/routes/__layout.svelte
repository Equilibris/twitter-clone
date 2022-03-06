<script lang="ts">
	import '../app.css';
	import Fetcher from '$lib/data/me/fetcher.svelte';
	import { me } from '$lib/data/me/store';
	import Anchor from '$lib/components/anchor.svelte';
	import SearchIcon from 'carbon-icons-svelte/lib/Search20';

	const signOut = async (e: Event) => {
		e.preventDefault();

		if (window.sessionStorage && window.localStorage) {
			window.sessionStorage.removeItem('user');
			window.localStorage.removeItem('refresh_token');
		}

		$me = null;
	};
</script>

<main class="dark:bg-slate-800 dark:text-white min-h-screen">
	<header class="w-full flex-row pt-3 px-3 box-border bold-text ">
		<div class="flex justify-between items-center border-b border-pink-400 pb-3 gap-4">
			<div class="font-bold hover:text-pink-200 transition">
				<Anchor href="/">Shwitter</Anchor>
			</div>
			<form
				action="/search"
				class="bg-pink-50 p-2 text-black flex-1 flex items-center gap-1 rounded-sm"
			>
				<SearchIcon />
				<input
					placeholder="search"
					class="bg-transparent flex-1 focus:outline-none"
					type="text"
					name="query"
				/>
			</form>
			<div class="flex gap-4 items-center">
				{#if $me}
					<a on:click={signOut} href="/">
						<div class="hover:text-pink-100 transition rounded-sm bg-pink-400 p-2 text-white">
							Sign out
						</div>
					</a>
				{:else}
					<Anchor href="/sign_in">
						<div class="hover:text-pink-200 transition">Sign in</div>
					</Anchor>
					<Anchor href="/sign_up">
						<div class="hover:text-pink-100 transition rounded-sm bg-pink-400 p-2 text-white">
							Sign up
						</div>
					</Anchor>
				{/if}
			</div>
		</div>
	</header>
	<Fetcher />
	<slot />
</main>
