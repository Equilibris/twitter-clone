<script lang="ts">
	import Encapsulator from '$lib/components/encapsulator.svelte';
	import type { Paths } from '$lib/typings/api';
	import { post } from '$lib/utils/fetch';
	import { me } from '$lib/data/me/store';

	let password = '';
	let name = '';

	let error = '';

	const handleSubmit = async (e: Event) => {
		e.preventDefault();

		const result = await post<Paths['user']['sign_in']>('/user/sign_in', {
			name,
			password
		});

		me.set(result.data);

		if (result.data && window.location) window.location.assign('/');
		else if (result.error?.[1]) error = result.error[1];
	};
</script>

<Encapsulator>
	<div class="container">
		<form on:submit={handleSubmit}>
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
	</div>
</Encapsulator>

<style lang="scss">
	form {
		display: flex;
		flex-direction: column;
	}

	.container {
		display: flex;
		justify-content: center;
		align-items: center;
	}
</style>
