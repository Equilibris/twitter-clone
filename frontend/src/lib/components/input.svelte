<script lang="ts">
	export let value = '';
	export const name = '';
	export const type = 'text';
</script>

<label for={name}>name</label>
<input {type} {name} bind:value={value} />
