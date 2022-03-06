<script lang="ts">
	import { paths } from '$lib/utils/fetch';
	import { me } from '$lib/data/me/store';
	import CenterContainer from '$lib/components/centerContainer.svelte';

	let password = '';
	let repassword = '';
	let name = '';

	let error = '';

	const handleSubmit = async (e: Event) => {
		e.preventDefault();

		if (password !== repassword) {
			error = 'Passwords do not match';

			return;
		}

		const result = await paths.user.signUp({
			name,
			password,
		});

		me.set(result.data);

		if (result.data && window.location) window.location.assign('/');
		else if (result.error?.[1]) {
			for (const e in result.error[1]) {
				error = result.error[1][e];
			}
		}
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
		<div class="flex flex-col bg-pink-50 p-2 rounded text-black">
			<label for="repassword" class="font-bold">Repeat Password</label>
			<input
				required
				class="bg-transparent focus:outline-none"
				placeholder="Sorry come again?"
				type="password"
				name="repassword"
				bind:value={repassword}
			/>
		</div>

		<button class="p-2 bg-pink-400 rounded hover:text-pink-200 transition">Sign up</button>

		{#if error}
			<p class="error">
				{error}
			</p>
		{/if}
	</form>
</CenterContainer>
