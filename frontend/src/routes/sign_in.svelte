<script lang="ts">
	import { paths } from '$lib/utils/fetch';
	import { me } from '$lib/data/me/store';
	import CenterContainer from '$lib/components/centerContainer.svelte';

	let password = '';
	let name = '';

	let error = '';

	const handleSubmit = async (e: Event) => {
		e.preventDefault();

		const result = await paths.user.signIn({
			name,
			password,
		});

		me.set(result.data);

		if (result.data && window.location) window.location.assign('/');
		else if (result.error?.[1]) error = result.error[1];
	};
</script>

<CenterContainer>
	<form on:submit={handleSubmit} class="flex flex-col gap-3">
		<div class="flex flex-col bg-pink-50 p-2 rounded text-black">
			<label for="name" class="font-bold">Name</label>
			<input
				required
				placeholder="Username"
				class="bg-transparent focus:outline-none"
				type="text"
				name="name"
				bind:value={name}
			/>
		</div>
		<div class="flex flex-col bg-pink-50 p-2 rounded text-black">
			<label for="password" class="font-bold">Password</label>
			<input
				required
				class="bg-transparent focus:outline-none"
				placeholder="Psw1"
				type="password"
				name="password"
				bind:value={password}
			/>
		</div>
		<button class="p-2 bg-pink-400 rounded hover:text-pink-200 transition">Sign in</button>

		{#if error}
			<p class="error">
				{error}
			</p>
		{/if}
	</form>
</CenterContainer>
