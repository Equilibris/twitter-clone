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
	<form on:submit={handleSubmit} class="flex flex-col">
		<label for="name">name</label>
		<input type="text" name="name" bind:value={name} />
		<label for="password">password </label>
		<input type="password" name="password" bind:value={password} />

		<button>Sign in</button>

		{#if error}
			<p class="error">
				{error}
			</p>
		{/if}
	</form>
</CenterContainer>
