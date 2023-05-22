<script lang="ts">
	import TopAppBar, {
	  Row,
	  Section,
	  Title,
	  AutoAdjust,
	} from '@smui/top-app-bar';

	import PageTransition from '$lib/components/PageTransition.svelte';
  
	let topAppBar: TopAppBar;
	export let data: any;
  </script>

<div>
	<TopAppBar bind:this={topAppBar} variant="fixed">
		<Row>
		  <Section>
			<a href="/">
			  <Title>Home</Title>
			</a>
			<a href="/recipie">
			  <Title>Recipie</Title>
			</a>
	
		  </Section>
		</Row>
	  </TopAppBar>
	  <AutoAdjust {topAppBar}>
		<PageTransition pathname={data.pathname}>
		  <slot />
		</PageTransition>
	  </AutoAdjust>

</div>
  

  
  <style>
	/* Hide everything above this component. */
	:global(#smui-app),
	:global(body),
	:global(html) {
	  display: block !important;
	  height: auto !important;
	  width: auto !important;
	  position: static !important;
	  margin: 0 !important;
	}
  </style>
  