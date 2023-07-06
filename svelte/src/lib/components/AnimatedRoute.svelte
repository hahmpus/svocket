<script>
	import { fly } from 'svelte/transition';
	export let key = '';
</script>

{#key key}
	<div in:fly={{ y: 5, duration: 75, delay: 150 }} out:fly={{ y: 5, duration: 75 }}>
		<slot />
	</div>
{/key}