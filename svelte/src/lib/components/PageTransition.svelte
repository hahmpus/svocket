<script>
	import { fly } from 'svelte/transition';
	export let pathname = '';
</script>

{#key pathname}
	<div in:fly={{ y: 10, duration: 100, delay: 100 }} out:fly={{ y: 10, duration: 100 }}>
		<slot />
	</div>
{/key}