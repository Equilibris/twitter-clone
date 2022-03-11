<script lang="ts">
	import SendIcon from 'carbon-icons-svelte/lib/SendFilled20';

	import { createEventDispatcher } from 'svelte';

	const dispatch = createEventDispatcher<{ submit: null }>();

	export let value = '';

	export let placeholder = "What's on your mind?";
</script>

<form
	on:submit={(e) => {
		e.preventDefault();
		dispatch('submit');
	}}
	class="gap-1 flex flex-col bg-pink-50 text-black rounded p-2 m-1"
>
	<textarea
		{placeholder}
		class="bg-transparent focus:outline-none resize-none"
		name="message"
		cols="20"
		rows="7"
		bind:value
		on:keypress={(e) => {
			if (e.shiftKey && e.keyCode === 13) {
				e.preventDefault();
				dispatch('submit');
			}
		}}
	/>
	<button class="ml-auto bg-pink-400 h-10 w-10 flex items-center justify-center rounded-full"
		><SendIcon class="fill-white transition hover:fill-pink-200" /></button
	>
</form>
