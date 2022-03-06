<script lang="ts">
	export let say = '';
	export let gif = false;

	export let type: 'sm' | 'md' | 'sq' | 'or' = 'or';

	export let width = 0
	export let height = 0
</script>

<img
	src={`https://cataas.com/c${gif ? '/gif' : ''}${say ? `/s/${encodeURIComponent(say)}` : ''}?type=${type}${height ? `&height=${height}`:''}${width ? `&width=${width}`:''}`}
	alt={`${gif ? 'A gif' : 'An image'} of a cat${say ? ` saying ${say}` : ''}`}
/>
